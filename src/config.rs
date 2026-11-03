// SPDX-License-Identifier: GPL-3.0-or-later
//! The settings of the library. Every tuned value is a line of the settings file; a missing line keeps the conservative default.

use crate::error::ThlaError;

/// The settings.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    /// The most bytes an input file may have.
    pub max_file_bytes: u64,
    /// The resistivity of copper at 20 degrees Celsius, in ohm metres.
    pub resistivity_ohm_m: f64,
    /// The temperature coefficient of that resistivity per kelvin.
    pub temp_coeff_per_k: f64,
    /// The highest temperature, in degrees Celsius, at which the linear resistivity model is used; a case that gets hotter produces no number.
    pub max_temperature_c: f64,
    /// The iteration stops when the temperature changes by less than this many kelvin.
    pub convergence_k: f64,
    /// The most steps of the iteration.
    pub max_iterations: usize,
    /// The uncertainty, in percent, written into every estimate. It is a placeholder of the author and has not been measured on a bench.
    pub uncertainty_percent: f64,
    /// The most elements a network may hold.
    pub max_elements: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            max_file_bytes: 1024 * 1024,
            resistivity_ohm_m: 1.724e-8,
            temp_coeff_per_k: 0.00393,
            max_temperature_c: 125.0,
            convergence_k: 1e-9,
            max_iterations: 500,
            uncertainty_percent: 50.0,
            max_elements: 1000,
        }
    }
}

impl Config {
    /// Parse a settings file (`key = value` lines, `#` comments).
    pub fn parse(text: &str) -> Result<Config, ThlaError> {
        let mut config = Config::default();
        for (index, raw) in text.lines().enumerate() {
            let line = index + 1;
            let raw = raw.trim();
            if raw.is_empty() || raw.starts_with('#') {
                continue;
            }
            let (key, value) = raw
                .split_once('=')
                .ok_or(ThlaError::ConfigSyntax { line })?;
            let (key, value) = (key.trim(), value.trim());
            let bad = || ThlaError::ConfigBadValue {
                line,
                key: key.to_string(),
            };
            let positive = |v: &str| -> Result<f64, ThlaError> {
                let n: f64 = v.parse().map_err(|_| bad())?;
                if n.is_finite() && n > 0.0 {
                    Ok(n)
                } else {
                    Err(bad())
                }
            };
            match key {
                "max_file_bytes" => config.max_file_bytes = value.parse().map_err(|_| bad())?,
                "resistivity_ohm_m" => config.resistivity_ohm_m = positive(value)?,
                "temp_coeff_per_k" => config.temp_coeff_per_k = positive(value)?,
                "max_temperature_c" => config.max_temperature_c = positive(value)?,
                "convergence_k" => config.convergence_k = positive(value)?,
                "uncertainty_percent" => config.uncertainty_percent = positive(value)?,
                "max_iterations" | "max_elements" => {
                    let n: usize = value.parse().map_err(|_| bad())?;
                    if n == 0 {
                        return Err(bad());
                    }
                    if key == "max_iterations" {
                        config.max_iterations = n;
                    } else {
                        config.max_elements = n;
                    }
                }
                _ => {
                    return Err(ThlaError::ConfigUnknownKey {
                        line,
                        key: key.to_string(),
                    })
                }
            }
        }
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_and_changes() {
        let c = Config::parse("").unwrap();
        assert_eq!(c, Config::default());
        let c = Config::parse("# comment\nmax_temperature_c = 105\nuncertainty_percent = 30\n")
            .unwrap();
        assert_eq!(c.max_temperature_c, 105.0);
        assert_eq!(c.uncertainty_percent, 30.0);
    }

    #[test]
    fn bad_lines_are_refused() {
        assert!(matches!(
            Config::parse("x\n"),
            Err(ThlaError::ConfigSyntax { line: 1 })
        ));
        assert!(matches!(
            Config::parse("a = 1\n"),
            Err(ThlaError::ConfigUnknownKey { line: 1, .. })
        ));
        for bad in [
            "max_temperature_c = 0\n",
            "resistivity_ohm_m = nan\n",
            "max_iterations = 0\n",
            "uncertainty_percent = -1\n",
            "max_elements = x\n",
        ] {
            assert!(
                matches!(
                    Config::parse(bad),
                    Err(ThlaError::ConfigBadValue { line: 1, .. })
                ),
                "{bad:?}"
            );
        }
    }
}
