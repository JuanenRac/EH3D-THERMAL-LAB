// SPDX-License-Identifier: GPL-3.0-or-later
//! A simplified thermal network: elements in series, each one a conduction, a convection, a given resistance or a group of parallel branches.

use crate::config::Config;
use crate::error::ThlaError;
use crate::result::{Kind, ThermalResult, Validity};
use crate::trace::positive;
use serde::{Deserialize, Serialize};

const MAX_DEPTH: usize = 16;

/// One element of the network. Every number names its unit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Element {
    /// Conduction through a slab: `length / (k * area)`.
    Conduction {
        /// The length along the heat flow in millimetres.
        length_mm: f64,
        /// The thermal conductivity in watts per metre and kelvin.
        conductivity_w_mk: f64,
        /// The section in square millimetres.
        area_mm2: f64,
    },
    /// Convection from a surface: `1 / (h * area)`.
    Convection {
        /// The coefficient in watts per square metre and kelvin.
        coefficient_w_m2k: f64,
        /// The surface in square millimetres.
        area_mm2: f64,
    },
    /// A resistance given directly, in kelvin per watt (a datasheet figure).
    Resistance {
        /// The resistance in kelvin per watt.
        k_per_w: f64,
    },
    /// Branches in parallel.
    Parallel {
        /// The branches; each one is an element.
        branches: Vec<Element>,
    },
}

/// A network driven by a power.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkCase {
    /// The power that enters the first element, in watts.
    pub power_w: f64,
    /// The temperature at the far end in degrees Celsius.
    pub ambient_c: f64,
    /// The elements in series, from the source to the ambient.
    pub elements: Vec<Element>,
}

impl Element {
    fn count(&self) -> usize {
        match self {
            Element::Parallel { branches } => {
                1 + branches.iter().map(Element::count).sum::<usize>()
            }
            _ => 1,
        }
    }

    /// The thermal resistance of the element in kelvin per watt.
    pub fn resistance(&self, depth: usize) -> Result<f64, ThlaError> {
        if depth > MAX_DEPTH {
            return Err(ThlaError::TooLarge {
                what: "depth of the network",
            });
        }
        match self {
            Element::Conduction {
                length_mm,
                conductivity_w_mk,
                area_mm2,
            } => {
                positive("length_mm", *length_mm)?;
                positive("conductivity_w_mk", *conductivity_w_mk)?;
                positive("area_mm2", *area_mm2)?;
                Ok(length_mm * 1e-3 / (conductivity_w_mk * area_mm2 * 1e-6))
            }
            Element::Convection {
                coefficient_w_m2k,
                area_mm2,
            } => {
                positive("coefficient_w_m2k", *coefficient_w_m2k)?;
                positive("area_mm2", *area_mm2)?;
                Ok(1.0 / (coefficient_w_m2k * area_mm2 * 1e-6))
            }
            Element::Resistance { k_per_w } => {
                positive("k_per_w", *k_per_w)?;
                Ok(*k_per_w)
            }
            Element::Parallel { branches } => {
                if branches.is_empty() {
                    return Err(ThlaError::BadInput {
                        what: "a parallel group has no branches".into(),
                    });
                }
                let mut conductance = 0.0;
                for b in branches {
                    conductance += 1.0 / b.resistance(depth + 1)?;
                }
                Ok(1.0 / conductance)
            }
        }
    }
}

/// The temperature rise of the source above the ambient, in kelvin.
pub fn temperature_rise(case: &NetworkCase, config: &Config) -> Result<ThermalResult, ThlaError> {
    positive("power_w", case.power_w)?;
    if !case.ambient_c.is_finite() {
        return Err(ThlaError::BadInput {
            what: "ambient_c must be a finite number".into(),
        });
    }
    if case.elements.is_empty() {
        return Err(ThlaError::BadInput {
            what: "the network has no elements".into(),
        });
    }
    if case.elements.iter().map(Element::count).sum::<usize>() > config.max_elements {
        return Err(ThlaError::TooLarge {
            what: "elements of the network",
        });
    }
    let mut total = 0.0;
    for e in &case.elements {
        total += e.resistance(0)?;
    }
    let rise = case.power_w * total;
    if case.ambient_c < 0.0 || case.ambient_c + rise > config.max_temperature_c {
        return Err(ThlaError::OutOfValidity {
            what: format!(
                "the source would be at {} C, outside 0 to {} C where the material values are used",
                case.ambient_c + rise,
                config.max_temperature_c
            ),
        });
    }
    Ok(ThermalResult {
        quantity: "temperature_rise".into(),
        value: rise,
        unit: "K".into(),
        kind: Kind::Estimate,
        hypotheses: vec![
            "steady state".into(),
            "one-dimensional heat flow through each element".into(),
            "constant material properties and coefficients".into(),
            "no heat leaves between the elements".into(),
        ],
        boundary_conditions: vec![
            format!("{} W enter the first element", case.power_w),
            format!("the far end is at {} C", case.ambient_c),
            format!("total resistance {} K/W", total),
        ],
        materials: vec!["the conductivities and coefficients given in the case".into()],
        uncertainty_percent: config.uncertainty_percent,
        limitations: vec![
            "not compared with a solver or a measurement".into(),
            "the uncertainty is a placeholder set in the settings, not a measured value".into(),
            "spreading of the heat is not modelled".into(),
            "the range of validity was chosen by the author".into(),
        ],
        validity: Some(Validity {
            quantity: "source_temperature_c".into(),
            min: 0.0,
            max: config.max_temperature_c,
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slab(k: f64) -> Element {
        Element::Conduction {
            length_mm: 1.6,
            conductivity_w_mk: k,
            area_mm2: 100.0,
        }
    }

    #[test]
    fn a_slab_matches_the_closed_form() {
        // dT = P L / (k A) = 1 * 0.0016 / (0.3 * 0.0001)
        let case = NetworkCase {
            power_w: 1.0,
            ambient_c: 25.0,
            elements: vec![slab(0.3)],
        };
        let r = temperature_rise(&case, &Config::default()).unwrap();
        let exact = 0.0016 / (0.3 * 0.0001);
        assert!((r.value - exact).abs() < 1e-9, "{}", r.value);
        assert!(r.check().is_empty());
    }

    #[test]
    fn series_add_and_equal_branches_in_parallel_halve() {
        let cfg = Config::default();
        let one = Element::Resistance { k_per_w: 3.0 };
        let case = NetworkCase {
            power_w: 1.0,
            ambient_c: 25.0,
            elements: vec![
                one.clone(),
                Element::Parallel {
                    branches: vec![one.clone(), one.clone()],
                },
                Element::Convection {
                    coefficient_w_m2k: 10.0,
                    area_mm2: 50_000.0,
                },
            ],
        };
        let r = temperature_rise(&case, &cfg).unwrap();
        // 3 + 1.5 + 1 / (10 * 0.05) = 6.5
        assert!((r.value - 6.5).abs() < 1e-12, "{}", r.value);
    }

    #[test]
    fn a_hot_source_gives_no_number() {
        let case = NetworkCase {
            power_w: 100.0,
            ambient_c: 25.0,
            elements: vec![Element::Resistance { k_per_w: 5.0 }],
        };
        assert!(matches!(
            temperature_rise(&case, &Config::default()),
            Err(ThlaError::OutOfValidity { .. })
        ));
    }

    #[test]
    fn bad_networks_are_refused() {
        let cfg = Config::default();
        let mk = |elements| NetworkCase {
            power_w: 1.0,
            ambient_c: 25.0,
            elements,
        };
        assert!(matches!(
            temperature_rise(&mk(vec![]), &cfg),
            Err(ThlaError::BadInput { .. })
        ));
        assert!(matches!(
            temperature_rise(&mk(vec![Element::Parallel { branches: vec![] }]), &cfg),
            Err(ThlaError::BadInput { .. })
        ));
        assert!(matches!(
            temperature_rise(&mk(vec![slab(0.0)]), &cfg),
            Err(ThlaError::BadInput { .. })
        ));
        let small = Config {
            max_elements: 2,
            ..Config::default()
        };
        assert!(matches!(
            temperature_rise(&mk(vec![slab(1.0), slab(1.0), slab(1.0)]), &small),
            Err(ThlaError::TooLarge { .. })
        ));
        let mut deep = Element::Resistance { k_per_w: 1.0 };
        for _ in 0..20 {
            deep = Element::Parallel {
                branches: vec![deep],
            };
        }
        assert!(matches!(
            temperature_rise(&mk(vec![deep]), &cfg),
            Err(ThlaError::TooLarge { .. })
        ));
    }

    #[test]
    fn the_case_reads_from_json() {
        let text = r#"{"power_w":1,"ambient_c":25,"elements":[{"type":"resistance","k_per_w":2},{"type":"parallel","branches":[{"type":"resistance","k_per_w":4},{"type":"resistance","k_per_w":4}]}]}"#;
        let case: NetworkCase = serde_json::from_str(text).unwrap();
        let r = temperature_rise(&case, &Config::default()).unwrap();
        assert!((r.value - 4.0).abs() < 1e-12);
        let bad = text.replace("resistance", "resist");
        assert!(serde_json::from_str::<NetworkCase>(&bad).is_err());
    }
}
