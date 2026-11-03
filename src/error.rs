// SPDX-License-Identifier: GPL-3.0-or-later
//! The typed errors of the library. Nothing panics on bad input: every refusal is one of these.

use crate::Code;
use std::path::PathBuf;

/// Everything that can go wrong while reading input or computing.
#[derive(Debug, thiserror::Error)]
pub enum ThlaError {
    /// The configuration has a line that is not `key = value`.
    #[error("configuration line {line}: expected `key = value`")]
    ConfigSyntax {
        /// 1-based line number.
        line: usize,
    },
    /// The configuration names a key this version does not know.
    #[error("configuration line {line}: unknown key `{key}`")]
    ConfigUnknownKey {
        /// 1-based line number.
        line: usize,
        /// The key as written.
        key: String,
    },
    /// The configuration value cannot be used.
    #[error("configuration line {line}: bad value for `{key}`")]
    ConfigBadValue {
        /// 1-based line number.
        line: usize,
        /// The key as written.
        key: String,
    },
    /// The input is not valid JSON of the expected shape.
    #[error("the input cannot be read: {0}")]
    BadJson(String),
    /// A number of the input is not usable (not finite, zero or negative where it must be positive).
    #[error("{code}: {what}", code = Code::BadInput.as_str())]
    BadInput {
        /// Which value and why.
        what: String,
    },
    /// The case leaves the range where the model holds, so no number is produced.
    #[error("{code}: {what}", code = Code::OutOfValidity.as_str())]
    OutOfValidity {
        /// What left the range.
        what: String,
    },
    /// The iteration did not converge, so the run is inconclusive and no number is produced.
    #[error("{code}: the iteration did not converge in {iterations} steps", code = Code::NotConverged.as_str())]
    NotConverged {
        /// Steps taken.
        iterations: usize,
    },
    /// A bound of the configuration was exceeded.
    #[error("limit exceeded: {what}")]
    TooLarge {
        /// Which bound.
        what: &'static str,
    },
    /// A file could not be read.
    #[error("cannot read {path}: {source}")]
    Io {
        /// The path that failed (as given by the caller).
        path: PathBuf,
        /// The operating system error.
        #[source]
        source: std::io::Error,
    },
}
