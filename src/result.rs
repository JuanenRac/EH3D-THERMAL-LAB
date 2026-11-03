// SPDX-License-Identifier: GPL-3.0-or-later
//! A thermal result and the check that refuses one without its conditions.

use crate::error::ThlaError;
use crate::Code;
use serde::{Deserialize, Serialize};

/// What kind of number it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// A closed-form value with no model error beyond its hypotheses.
    Analytic,
    /// A value of a simplified model, not compared with a solver or a measurement.
    Estimate,
}

/// The range of an input quantity inside which the model is used.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Validity {
    /// The quantity, with its unit in the name.
    pub quantity: String,
    /// The lowest value.
    pub min: f64,
    /// The highest value.
    pub max: f64,
}

/// A result with everything a reader needs to judge it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThermalResult {
    /// What is given, such as `temperature_rise`.
    pub quantity: String,
    /// The value.
    pub value: f64,
    /// Its unit.
    pub unit: String,
    /// Analytic or estimate.
    pub kind: Kind,
    /// What was assumed.
    #[serde(default)]
    pub hypotheses: Vec<String>,
    /// The conditions at the boundary, with their numbers.
    #[serde(default)]
    pub boundary_conditions: Vec<String>,
    /// The materials and the property values used.
    #[serde(default)]
    pub materials: Vec<String>,
    /// The uncertainty of the value in percent.
    #[serde(default)]
    pub uncertainty_percent: f64,
    /// What the result does not cover.
    #[serde(default)]
    pub limitations: Vec<String>,
    /// The range where the model is used.
    #[serde(default)]
    pub validity: Option<Validity>,
}

/// Something missing in a result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// The code.
    pub code: Code,
    /// What it is about.
    pub subject: &'static str,
    /// What is wrong, in English.
    pub message: String,
}

impl ThermalResult {
    /// Everything that keeps this result from being accepted. An empty list means the result carries all its conditions (not that it is right).
    pub fn check(&self) -> Vec<Finding> {
        let mut out = Vec::new();
        let mut add = |subject: &'static str, message: &str| {
            out.push(Finding {
                code: Code::MissingConditions,
                subject,
                message: message.to_string(),
            })
        };
        if self.quantity.trim().is_empty() {
            add("quantity", "the result does not say what it is");
        }
        if !self.value.is_finite() {
            add("value", "the value is not a finite number");
        }
        if self.unit.trim().is_empty() {
            add("unit", "the result has no unit");
        }
        for (subject, list) in [
            ("hypotheses", &self.hypotheses),
            ("boundary_conditions", &self.boundary_conditions),
            ("materials", &self.materials),
            ("limitations", &self.limitations),
        ] {
            if list.is_empty() || list.iter().any(|s| s.trim().is_empty()) {
                add(subject, "the list is empty or has an empty entry");
            }
        }
        if !self.uncertainty_percent.is_finite() || self.uncertainty_percent <= 0.0 {
            add(
                "uncertainty_percent",
                "the uncertainty is missing: a bare number gives false confidence",
            );
        }
        match &self.validity {
            None => add("validity", "the range of validity is missing"),
            Some(v) => {
                if v.quantity.trim().is_empty()
                    || !v.min.is_finite()
                    || !v.max.is_finite()
                    || v.min > v.max
                {
                    add("validity", "the range of validity is not usable");
                }
            }
        }
        out
    }

    /// Read a result from JSON (unknown fields are refused).
    pub fn from_json(text: &str) -> Result<ThermalResult, ThlaError> {
        serde_json::from_str(text).map_err(|e| ThlaError::BadJson(e.to_string()))
    }

    /// The result as pretty JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}

#[cfg(test)]
pub(crate) fn complete() -> ThermalResult {
    ThermalResult {
        quantity: "temperature_rise".into(),
        value: 10.0,
        unit: "K".into(),
        kind: Kind::Estimate,
        hypotheses: vec!["h".into()],
        boundary_conditions: vec!["b".into()],
        materials: vec!["m".into()],
        uncertainty_percent: 50.0,
        limitations: vec!["l".into()],
        validity: Some(Validity {
            quantity: "temperature_c".into(),
            min: 0.0,
            max: 125.0,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_complete_result_passes() {
        assert!(complete().check().is_empty());
    }

    #[test]
    fn each_missing_condition_is_reported_with_the_code() {
        let mut r = complete();
        r.hypotheses.clear();
        r.boundary_conditions = vec![" ".into()];
        r.materials.clear();
        r.limitations.clear();
        r.uncertainty_percent = 0.0;
        r.validity = None;
        r.unit = String::new();
        r.value = f64::NAN;
        let subjects: Vec<&str> = r.check().iter().map(|f| f.subject).collect();
        for s in [
            "value",
            "unit",
            "hypotheses",
            "boundary_conditions",
            "materials",
            "limitations",
            "uncertainty_percent",
            "validity",
        ] {
            assert!(subjects.contains(&s), "{s}");
        }
        assert!(r
            .check()
            .iter()
            .all(|f| f.code.as_str() == "PHOENIX-THLA-001"));
    }

    #[test]
    fn an_inverted_range_is_not_usable() {
        let mut r = complete();
        r.validity.as_mut().unwrap().min = 200.0;
        assert_eq!(r.check().len(), 1);
    }

    #[test]
    fn json_round_trip_and_unknown_field() {
        let r = complete();
        assert_eq!(ThermalResult::from_json(&r.to_json()).unwrap(), r);
        let bad = r.to_json().replace("\"unit\"", "\"unitt\"");
        assert!(matches!(
            ThermalResult::from_json(&bad),
            Err(ThlaError::BadJson(_))
        ));
    }
}
