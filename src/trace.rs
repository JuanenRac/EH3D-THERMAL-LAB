// SPDX-License-Identifier: GPL-3.0-or-later
//! The Joule heating of a copper trace that cools by convection from its exposed surface, with the resistance that grows with the temperature.

use crate::config::Config;
use crate::error::ThlaError;
use crate::result::{Kind, ThermalResult, Validity};
use serde::{Deserialize, Serialize};

/// A trace and its surroundings. Every number names its unit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraceCase {
    /// The current in amperes (direct current or the RMS value).
    pub current_a: f64,
    /// The length in millimetres.
    pub length_mm: f64,
    /// The width in millimetres.
    pub width_mm: f64,
    /// The copper thickness in micrometres.
    pub thickness_um: f64,
    /// The temperature of the surrounding air in degrees Celsius.
    pub ambient_c: f64,
    /// The convection coefficient in watts per square metre and kelvin, given by the person who runs the case.
    pub convection_w_m2k: f64,
}

pub(crate) fn positive(name: &str, v: f64) -> Result<(), ThlaError> {
    if v.is_finite() && v > 0.0 {
        Ok(())
    } else {
        Err(ThlaError::BadInput {
            what: format!("{name} must be a finite number above zero"),
        })
    }
}

/// The temperature rise of the trace above the air, in kelvin.
///
/// The temperature is found by repeating `T = ambient + P(T) / (h A)` until it stops changing. If it gets hotter than `max_temperature_c` the case is out of
/// validity (this includes the runaway where the heating grows faster than the cooling); if the steps run out it is not converged. In both cases there is no number.
pub fn temperature_rise(case: &TraceCase, config: &Config) -> Result<ThermalResult, ThlaError> {
    positive("current_a", case.current_a)?;
    positive("length_mm", case.length_mm)?;
    positive("width_mm", case.width_mm)?;
    positive("thickness_um", case.thickness_um)?;
    positive("convection_w_m2k", case.convection_w_m2k)?;
    if !case.ambient_c.is_finite() {
        return Err(ThlaError::BadInput {
            what: "ambient_c must be a finite number".into(),
        });
    }
    if case.ambient_c < 0.0 || case.ambient_c >= config.max_temperature_c {
        return Err(ThlaError::OutOfValidity {
            what: format!(
                "the ambient temperature {} C is outside 0 to {} C",
                case.ambient_c, config.max_temperature_c
            ),
        });
    }
    let length = case.length_mm * 1e-3;
    let width = case.width_mm * 1e-3;
    let thickness = case.thickness_um * 1e-6;
    let section = width * thickness;
    // Top face and the two edges are exposed; the face on the board is not.
    let surface = (width + 2.0 * thickness) * length;
    let conductance = case.convection_w_m2k * surface;
    let resistance_at = |t: f64| {
        config.resistivity_ohm_m * (1.0 + config.temp_coeff_per_k * (t - 20.0)) * length / section
    };
    let mut t = case.ambient_c;
    let mut converged = false;
    for _ in 0..config.max_iterations {
        let power = case.current_a * case.current_a * resistance_at(t);
        let next = case.ambient_c + power / conductance;
        if !next.is_finite() || next > config.max_temperature_c {
            return Err(ThlaError::OutOfValidity {
                what: format!(
                    "the trace would pass {} C, where the resistivity model is not used",
                    config.max_temperature_c
                ),
            });
        }
        let done = (next - t).abs() < config.convergence_k;
        t = next;
        if done {
            converged = true;
            break;
        }
    }
    if !converged {
        return Err(ThlaError::NotConverged {
            iterations: config.max_iterations,
        });
    }
    Ok(ThermalResult {
        quantity: "temperature_rise".into(),
        value: t - case.ambient_c,
        unit: "K".into(),
        kind: Kind::Estimate,
        hypotheses: vec![
            "steady state".into(),
            "the whole trace is at one temperature".into(),
            "the heat leaves only by convection from the top face and the two edges; conduction into the board and radiation are ignored, so the rise is high".into(),
            "the current is uniform over the section".into(),
        ],
        boundary_conditions: vec![
            format!("air at {} C", case.ambient_c),
            format!("convection coefficient {} W/m2K, given by the user", case.convection_w_m2k),
            format!("{} A in a trace of {} mm by {} mm by {} um", case.current_a, case.length_mm, case.width_mm, case.thickness_um),
        ],
        materials: vec![format!(
            "copper, resistivity {} ohm m at 20 C, linear coefficient {} per K",
            config.resistivity_ohm_m, config.temp_coeff_per_k
        )],
        uncertainty_percent: config.uncertainty_percent,
        limitations: vec![
            "not compared with a solver or a measurement".into(),
            "the uncertainty is a placeholder set in the settings, not a measured value".into(),
            "no skin effect, so alternating currents of high frequency are not covered".into(),
            "the range of validity was chosen by the author".into(),
        ],
        validity: Some(Validity {
            quantity: "conductor_temperature_c".into(),
            min: 0.0,
            max: config.max_temperature_c,
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn case() -> TraceCase {
        TraceCase {
            current_a: 2.0,
            length_mm: 100.0,
            width_mm: 2.0,
            thickness_um: 35.0,
            ambient_c: 20.0,
            convection_w_m2k: 20.0,
        }
    }

    #[test]
    fn matches_the_closed_form_when_the_air_is_at_the_reference_temperature() {
        let c = case();
        let cfg = Config::default();
        let length = c.length_mm * 1e-3;
        let section = c.width_mm * 1e-3 * c.thickness_um * 1e-6;
        let surface = (c.width_mm * 1e-3 + 2.0 * c.thickness_um * 1e-6) * length;
        let g = c.convection_w_m2k * surface;
        let p0 = c.current_a * c.current_a * cfg.resistivity_ohm_m * length / section;
        let exact = p0 / (g - cfg.temp_coeff_per_k * p0);
        let r = temperature_rise(&c, &cfg).unwrap();
        assert!(
            (r.value - exact).abs() < 1e-6 * exact,
            "{} {}",
            r.value,
            exact
        );
        assert!(r.check().is_empty());
        assert_eq!(r.kind, Kind::Estimate);
    }

    #[test]
    fn more_current_gives_more_rise_and_a_better_cooling_gives_less() {
        let cfg = Config::default();
        let a = temperature_rise(&case(), &cfg).unwrap().value;
        let mut more = case();
        more.current_a = 3.0;
        assert!(temperature_rise(&more, &cfg).unwrap().value > a);
        let mut cool = case();
        cool.convection_w_m2k = 40.0;
        assert!(temperature_rise(&cool, &cfg).unwrap().value < a);
    }

    #[test]
    fn a_hot_case_gives_no_number() {
        let mut c = case();
        c.current_a = 50.0;
        assert!(matches!(
            temperature_rise(&c, &Config::default()),
            Err(ThlaError::OutOfValidity { .. })
        ));
    }

    #[test]
    fn running_out_of_steps_is_not_converged() {
        let cfg = Config {
            max_iterations: 1,
            ..Config::default()
        };
        assert!(matches!(
            temperature_rise(&case(), &cfg),
            Err(ThlaError::NotConverged { iterations: 1 })
        ));
    }

    #[test]
    fn bad_inputs_are_refused() {
        let cfg = Config::default();
        for edit in [
            |c: &mut TraceCase| c.current_a = 0.0,
            |c: &mut TraceCase| c.length_mm = -1.0,
            |c: &mut TraceCase| c.width_mm = f64::NAN,
            |c: &mut TraceCase| c.thickness_um = f64::INFINITY,
            |c: &mut TraceCase| c.convection_w_m2k = 0.0,
            |c: &mut TraceCase| c.ambient_c = f64::NAN,
        ] {
            let mut c = case();
            edit(&mut c);
            assert!(matches!(
                temperature_rise(&c, &cfg),
                Err(ThlaError::BadInput { .. })
            ));
        }
        let mut c = case();
        c.ambient_c = 130.0;
        assert!(matches!(
            temperature_rise(&c, &cfg),
            Err(ThlaError::OutOfValidity { .. })
        ));
    }
}
