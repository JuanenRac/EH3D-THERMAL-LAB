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
