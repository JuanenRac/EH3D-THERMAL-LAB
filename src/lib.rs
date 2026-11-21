// SPDX-License-Identifier: GPL-3.0-or-later
//! EH3D-THERMAL-LAB: stage-1 thermal estimates.
//!
//! Two things are computed here: the temperature rise of a copper trace that carries a current (the Joule effect, with the resistance that grows with the
//! temperature), and the rise across a simplified thermal network of series and parallel resistances. A [`ThermalResult`] can only be accepted with its
//! hypotheses, boundary conditions, materials, uncertainty, limitations, kind and range of validity. A run that leaves the range of validity or does not
//! converge is an error, never a number.
//!
//! Nothing here has been compared with a measurement or a solver, so every result is an estimate. This crate is not a safety system and never starts
//! motion: the thermal protection of the hardware stays authoritative.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod cli;
pub mod config;
pub mod error;
pub mod network;
pub mod result;
pub mod trace;

pub use error::ThlaError;
pub use result::ThermalResult;

/// The stable code of a finding. Codes are never reused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Code {
    /// A result lacks hypotheses, boundary conditions, materials, limitations, a unit or an uncertainty.
    MissingConditions,
    /// The case is outside the range where the model holds.
    OutOfValidity,
    /// The iteration did not converge.
    NotConverged,
    /// An input number is not usable.
    BadInput,
}

impl Code {
    /// The text of the code, such as `PHOENIX-THLA-001`.
    pub fn as_str(self) -> &'static str {
        match self {
            Code::MissingConditions => "PHOENIX-THLA-001",
            Code::OutOfValidity => "PHOENIX-THLA-002",
            Code::NotConverged => "PHOENIX-THLA-003",
            Code::BadInput => "PHOENIX-THLA-004",
        }
    }
}

/// The simulation of a bench: a thermocouple on the trace, made of the model itself with an offset, noise, a drift, dropouts, a sensor that sticks or saturates, and the
/// ramp of the current that a person would run, ending at the first current the model refuses (a bench would cut the current there). It lets the comparison of a result
/// with a reading be tried on every kind of bad reading before there is a bench. The "plant" is the model, so a verdict here says whether the comparison notices a bad
/// sensor; it says nothing about how good the model is. Nothing here was measured.
pub mod sim {
    use crate::config::Config;
    use crate::trace::{temperature_rise, TraceCase};

    /// A small deterministic generator (xorshift64*). It reads no clock.
    #[derive(Debug, Clone)]
    pub struct SimRng(u64);

    impl SimRng {
        /// A generator for the seed (zero is replaced, because xorshift cannot start from zero).
        pub fn new(seed: u64) -> SimRng {
            SimRng(if seed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                seed
            })
        }

        /// A number below `n` (`n` must be above zero).
        pub fn below(&mut self, n: u64) -> u64 {
            let mut x = self.0;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.0 = x;
            x.wrapping_mul(0x2545_F491_4F6C_DD1D) % n
        }
    }

    /// A simulated thermocouple. Every fault is off until its field is set.
    #[derive(Debug, Clone)]
    pub struct SimThermocouple {
        rng: SimRng,
        samples: usize,
        last: Option<f64>,
        /// Half the width of the noise in kelvin (each reading is off by a number in the range, from the seed).
        pub noise_k: f64,
        /// A fixed error in kelvin (a bad calibration or a bad contact).
        pub offset_k: f64,
        /// An error that grows by this many kelvin with every reading.
        pub drift_k_per_sample: f64,
        /// The chance in a thousand that a reading is not delivered.
        pub dropout_per_mille: u64,
        /// The first readings that are not delivered (the sensor is not connected yet).
        pub dead_first: usize,
        /// From this reading on the sensor repeats its last value.
        pub stuck_after: Option<usize>,
        /// The most the sensor can read, in kelvin above the air.
        pub saturates_at_k: f64,
    }

    impl SimThermocouple {
        /// A sensor with no fault: it reads what the model says.
        pub fn perfect(seed: u64) -> SimThermocouple {
            SimThermocouple {
                rng: SimRng::new(seed),
                samples: 0,
                last: None,
                noise_k: 0.0,
                offset_k: 0.0,
                drift_k_per_sample: 0.0,
                dropout_per_mille: 0,
                dead_first: 0,
                stuck_after: None,
                saturates_at_k: f64::INFINITY,
            }
        }

        /// One reading of a trace whose true rise is `true_rise_k`; `None` when it is not delivered.
        pub fn read(&mut self, true_rise_k: f64) -> Option<f64> {
            let n = self.samples;
            self.samples += 1;
            let dropped = self.rng.below(1000) < self.dropout_per_mille;
            let noise = (self.rng.below(2001) as f64 - 1000.0) / 1000.0 * self.noise_k;
            if n < self.dead_first || dropped {
                return None;
            }
            if let (Some(from), Some(last)) = (self.stuck_after, self.last) {
                if n >= from {
                    return Some(last);
                }
            }
            let value = (true_rise_k + self.offset_k + self.drift_k_per_sample * n as f64 + noise)
                .min(self.saturates_at_k);
            self.last = Some(value);
            Some(value)
        }
    }

    /// What the comparison of the model with the reading says.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Verdict {
        /// The reading is inside the uncertainty of the result (and the noise of the sensor).
        Agrees,
        /// The sensor reads more than the model allows: the model says too little.
        ModelLow,
        /// The sensor reads less than the model allows: the model says too much.
        ModelHigh,
        /// The model gave a number and the sensor gave none.
        NoReading,
        /// The model refused the case, so there is nothing to compare.
        NoModelNumber,
    }

    /// One step of the ramp.
    #[derive(Debug, Clone, PartialEq)]
    pub struct RampPoint {
        /// The current of the step in amperes.
        pub current_a: f64,
        /// The rise the model gave in kelvin, if it gave a number.
        pub model_k: Option<f64>,
        /// The rise the sensor read in kelvin, if it delivered one.
        pub reading_k: Option<f64>,
        /// What the comparison says.
        pub verdict: Verdict,
    }

    /// The verdict for a model value and a reading, with the uncertainty of the result in percent and the noise of the sensor in kelvin.
    pub fn judge(
        model_k: Option<f64>,
        reading_k: Option<f64>,
        uncertainty_percent: f64,
        noise_k: f64,
    ) -> Verdict {
        match (model_k, reading_k) {
            (None, _) => Verdict::NoModelNumber,
            (Some(_), None) => Verdict::NoReading,
            (Some(model), Some(reading)) => {
                let tolerance = model.abs() * uncertainty_percent / 100.0 + noise_k;
                if reading > model + tolerance {
                    Verdict::ModelLow
                } else if reading < model - tolerance {
                    Verdict::ModelHigh
                } else {
                    Verdict::Agrees
                }
            }
        }
    }

    /// Run the currents in order on the trace with the sensor; the ramp ends at the first current the model refuses (that point is in the list, with no numbers).
    pub fn ramp(
        case: &TraceCase,
        config: &Config,
        currents_a: &[f64],
        sensor: &mut SimThermocouple,
    ) -> Vec<RampPoint> {
        let mut points = Vec::new();
        for &current in currents_a {
            let mut step = case.clone();
            step.current_a = current;
            match temperature_rise(&step, config) {
                Ok(result) => {
                    let reading = sensor.read(result.value);
                    points.push(RampPoint {
                        current_a: current,
                        model_k: Some(result.value),
                        reading_k: reading,
                        verdict: judge(
                            Some(result.value),
                            reading,
                            result.uncertainty_percent,
                            sensor.noise_k,
                        ),
                    });
                }
                Err(_) => {
                    points.push(RampPoint {
                        current_a: current,
                        model_k: None,
                        reading_k: None,
                        verdict: Verdict::NoModelNumber,
                    });
                    break;
                }
            }
        }
        points
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn case() -> TraceCase {
            TraceCase {
                current_a: 1.0,
                length_mm: 100.0,
                width_mm: 2.0,
                thickness_um: 35.0,
                ambient_c: 20.0,
                convection_w_m2k: 20.0,
            }
        }

        fn verdicts(points: &[RampPoint]) -> Vec<Verdict> {
            points.iter().map(|p| p.verdict).collect()
        }

        const SAFE: [f64; 4] = [0.5, 1.0, 2.0, 3.0];

        #[test]
        fn a_perfect_sensor_agrees_with_the_model_at_every_step() {
            let points = ramp(
                &case(),
                &Config::default(),
                &SAFE,
                &mut SimThermocouple::perfect(1),
            );
            assert_eq!(verdicts(&points), vec![Verdict::Agrees; 4]);
            assert!(points.iter().all(|p| p.model_k == p.reading_k));
            assert!(points.windows(2).all(|w| w[0].model_k < w[1].model_k));
        }

        #[test]
        fn the_ramp_ends_at_the_first_current_the_model_refuses_and_the_sensor_is_not_read_there() {
            let currents = [0.5, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
            let mut sensor = SimThermocouple::perfect(1);
            let points = ramp(&case(), &Config::default(), &currents, &mut sensor);
            let last = points.last().unwrap();
            assert_eq!(last.verdict, Verdict::NoModelNumber);
            assert_eq!((last.model_k, last.reading_k), (None, None));
            assert!(points.len() < currents.len());
            assert!(points[..points.len() - 1]
                .iter()
                .all(|p| p.verdict == Verdict::Agrees));
        }

        #[test]
        fn an_offset_in_either_direction_is_noticed_as_the_model_being_low_or_high() {
            for (offset, expected) in [(1000.0, Verdict::ModelLow), (-1000.0, Verdict::ModelHigh)] {
                let mut sensor = SimThermocouple::perfect(1);
                sensor.offset_k = offset;
                let points = ramp(&case(), &Config::default(), &SAFE, &mut sensor);
                assert_eq!(verdicts(&points), vec![expected; 4], "{offset}");
            }
        }

        #[test]
        fn a_sensor_that_sticks_is_caught_once_the_model_has_moved_away_from_it() {
            let mut sensor = SimThermocouple::perfect(1);
            sensor.stuck_after = Some(1);
            let points = ramp(&case(), &Config::default(), &SAFE, &mut sensor);
            assert_eq!(points[0].verdict, Verdict::Agrees);
            assert_eq!(points[3].verdict, Verdict::ModelHigh);
            assert!(points[1..]
                .iter()
                .all(|p| p.reading_k == points[0].reading_k));
        }

        #[test]
        fn a_sensor_that_saturates_is_caught_when_the_model_goes_above_its_range() {
            let mut sensor = SimThermocouple::perfect(1);
            sensor.saturates_at_k = 20.0;
            let points = ramp(&case(), &Config::default(), &SAFE, &mut sensor);
            assert_eq!(points[0].verdict, Verdict::Agrees);
            assert_eq!(points[3].verdict, Verdict::ModelHigh);
            assert_eq!(points[3].reading_k, Some(20.0));
        }

        #[test]
        fn a_drift_is_noticed_late_and_a_dead_sensor_that_comes_alive_recovers() {
            let mut drifting = SimThermocouple::perfect(1);
            drifting.drift_k_per_sample = 10.0;
            let points = ramp(&case(), &Config::default(), &[1.0; 8], &mut drifting);
            assert_eq!(points[0].verdict, Verdict::Agrees);
            assert_eq!(points[7].verdict, Verdict::ModelLow);
            let first_bad = points
                .iter()
                .position(|p| p.verdict != Verdict::Agrees)
                .unwrap();
            assert!(points[first_bad..]
                .iter()
                .all(|p| p.verdict == Verdict::ModelLow));

            let mut late = SimThermocouple::perfect(1);
            late.dead_first = 2;
            let points = ramp(&case(), &Config::default(), &SAFE, &mut late);
            assert_eq!(
                verdicts(&points),
                vec![
                    Verdict::NoReading,
                    Verdict::NoReading,
                    Verdict::Agrees,
                    Verdict::Agrees
                ]
            );
        }

        #[test]
        fn a_seeded_campaign_of_noise_and_dropouts_never_calls_a_good_reading_bad_and_repeats_exactly(
        ) {
            let run = |seed: u64| {
                let mut sensor = SimThermocouple::perfect(seed);
                sensor.noise_k = 0.5;
                sensor.dropout_per_mille = 300;
                ramp(&case(), &Config::default(), &[1.0; 20], &mut sensor)
            };
            let mut dropped = 0;
            for seed in 1..=30 {
                let points = run(seed);
                assert_eq!(points, run(seed), "seed {seed}");
                for p in &points {
                    match p.reading_k {
                        None => {
                            dropped += 1;
                            assert_eq!(p.verdict, Verdict::NoReading, "seed {seed}");
                        }
                        Some(r) => {
                            assert!((r - p.model_k.unwrap()).abs() <= 0.5 + 1e-9, "seed {seed}");
                            assert_eq!(p.verdict, Verdict::Agrees, "seed {seed}");
                        }
                    }
                }
            }
            assert!(dropped > 0, "no reading was ever dropped");
            assert_ne!(run(1), run(2));
        }

        #[test]
        fn the_judge_has_a_verdict_for_every_pair_and_the_generator_survives_a_zero_seed() {
            assert_eq!(judge(None, Some(1.0), 50.0, 0.0), Verdict::NoModelNumber);
            assert_eq!(judge(Some(1.0), None, 50.0, 0.0), Verdict::NoReading);
            assert_eq!(judge(Some(10.0), Some(15.0), 50.0, 0.0), Verdict::Agrees);
            assert_eq!(judge(Some(10.0), Some(15.1), 50.0, 0.0), Verdict::ModelLow);
            assert_eq!(judge(Some(10.0), Some(4.9), 50.0, 0.0), Verdict::ModelHigh);
            assert_eq!(judge(Some(10.0), Some(16.0), 50.0, 1.0), Verdict::Agrees);
            let mut rng = SimRng::new(0);
            let drawn: Vec<u64> = (0..8).map(|_| rng.below(1_000_000)).collect();
            assert!(drawn.windows(2).any(|w| w[0] != w[1]));
        }

        #[test]
        fn the_judge_stops_exactly_on_the_edge_of_the_tolerance() {
            let edge = |reading: f64| judge(Some(100.0), Some(reading), 10.0, 1.0);
            assert_eq!(edge(111.0), Verdict::Agrees);
            assert_eq!(edge(111.0001), Verdict::ModelLow);
            assert_eq!(edge(89.0), Verdict::Agrees);
            assert_eq!(edge(88.9999), Verdict::ModelHigh);
            assert_eq!(judge(Some(5.0), Some(5.0), 0.0, 0.0), Verdict::Agrees);
        }

        #[test]
        fn the_tolerance_of_a_negative_model_value_is_the_same_as_of_its_opposite() {
            assert_eq!(
                judge(Some(-100.0), Some(-111.0), 10.0, 1.0),
                Verdict::Agrees
            );
            assert_eq!(
                judge(Some(-100.0), Some(-120.0), 10.0, 1.0),
                Verdict::ModelHigh
            );
        }

        #[test]
        fn the_codes_are_written_in_order_and_none_repeats() {
            use crate::Code;
            let all = [
                Code::MissingConditions,
                Code::OutOfValidity,
                Code::NotConverged,
                Code::BadInput,
            ];
            let texts: Vec<&str> = all.iter().map(|c| c.as_str()).collect();
            assert_eq!(
                texts,
                [
                    "PHOENIX-THLA-001",
                    "PHOENIX-THLA-002",
                    "PHOENIX-THLA-003",
                    "PHOENIX-THLA-004"
                ]
            );
        }

        #[test]
        fn a_ramp_without_currents_is_empty_and_one_with_a_single_current_has_one_point() {
            let cfg = Config::default();
            let mut sensor = SimThermocouple::perfect(7);
            assert!(ramp(&case(), &cfg, &[], &mut sensor).is_empty());
            let one = ramp(&case(), &cfg, &[1.5], &mut sensor);
            assert_eq!(one.len(), 1);
            assert_eq!(one[0].current_a, 1.5);
        }

        #[test]
        fn the_points_of_a_ramp_keep_the_order_of_the_currents() {
            let points = ramp(
                &case(),
                &Config::default(),
                &SAFE,
                &mut SimThermocouple::perfect(3),
            );
            let currents: Vec<f64> = points.iter().map(|p| p.current_a).collect();
            assert_eq!(currents, SAFE);
        }

        #[test]
        fn the_generator_gives_the_same_numbers_for_the_same_seed_and_stays_below_the_bound() {
            let mut a = SimRng::new(42);
            let mut b = SimRng::new(42);
            let mut c = SimRng::new(43);
            let first: Vec<u64> = (0..64).map(|_| a.below(1000)).collect();
            let second: Vec<u64> = (0..64).map(|_| b.below(1000)).collect();
            let other: Vec<u64> = (0..64).map(|_| c.below(1000)).collect();
            assert_eq!(first, second);
            assert_ne!(first, other);
            assert!(first.iter().all(|&n| n < 1000));
            assert_eq!(SimRng::new(9).below(1), 0);
        }
    }
}
