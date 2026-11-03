// SPDX-License-Identifier: GPL-3.0-or-later
//! The command line. It is a thin layer: parsing the arguments and printing; the calculations live in the other modules.
//!
//! Exit codes: 0 done, 1 a result lacks its conditions or the case gives no number, 2 the input or the arguments are unusable.

use crate::config::Config;
use crate::error::ThlaError;
use crate::network::{self, NetworkCase};
use crate::result::ThermalResult;
use crate::trace::{self, TraceCase};
use std::fs;
use std::io::Write;
use std::path::Path;

const USAGE: &str = "\
eh3d-thermal-lab [--config FILE] <command>

  trace <case.json>      temperature rise of a copper trace with a current (Joule effect, convection)
  network <case.json>    temperature rise across a series and parallel network of thermal resistances
  check <result.json>    list what a result lacks: hypotheses, boundary conditions, materials, uncertainty, limitations, range of validity

Each calculation prints the result as JSON with all its conditions.
Exit codes: 0 done, 1 missing conditions or no number for the case, 2 unusable input.
";

fn read_text(path: &Path, limit: u64) -> Result<String, ThlaError> {
    let io = |source| ThlaError::Io {
        path: path.to_path_buf(),
        source,
    };
    if fs::metadata(path).map_err(io)?.len() > limit {
        return Err(ThlaError::TooLarge { what: "input file" });
    }
    fs::read_to_string(path).map_err(io)
}

/// Run the command line with the given arguments (without the program name). Returns the exit code.
pub fn run(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    match dispatch(args, out) {
        Ok(code) => code,
        Err(None) => {
            let _ = err.write_all(USAGE.as_bytes());
            2
        }
        Err(Some(error)) => {
            let _ = writeln!(err, "error: {error}");
            // A case that gives no number is a result of the calculation, not a bad input.
            match error {
                ThlaError::OutOfValidity { .. } | ThlaError::NotConverged { .. } => 1,
                _ => 2,
            }
        }
    }
}

fn dispatch(args: &[String], out: &mut dyn Write) -> Result<i32, Option<ThlaError>> {
    let mut args = args.to_vec();
    let mut config = Config::default();
    if args.first().map(String::as_str) == Some("--config") {
        if args.len() < 3 {
            return Err(None);
        }
        config = Config::parse(&read_text(Path::new(&args[1]), 1024 * 1024).map_err(Some)?)
            .map_err(Some)?;
        args.drain(..2);
    }
    let rest: Vec<&str> = args.iter().map(String::as_str).collect();
    let text = |path: &str| read_text(Path::new(path), config.max_file_bytes).map_err(Some);
    let bad_json = |e: serde_json::Error| Some(ThlaError::BadJson(e.to_string()));
    match rest.as_slice() {
        ["trace", path] => {
            let case: TraceCase = serde_json::from_str(&text(path)?).map_err(bad_json)?;
            let result = trace::temperature_rise(&case, &config).map_err(Some)?;
            let _ = writeln!(out, "{}", result.to_json());
            Ok(0)
        }
        ["network", path] => {
            let case: NetworkCase = serde_json::from_str(&text(path)?).map_err(bad_json)?;
            let result = network::temperature_rise(&case, &config).map_err(Some)?;
            let _ = writeln!(out, "{}", result.to_json());
            Ok(0)
        }
        ["check", path] => {
            let result = ThermalResult::from_json(&text(path)?).map_err(Some)?;
            let findings = result.check();
            for f in &findings {
                let _ = writeln!(out, "{} [{}]: {}", f.code.as_str(), f.subject, f.message);
            }
            let _ = writeln!(out, "# {} findings", findings.len());
            Ok(if findings.is_empty() { 0 } else { 1 })
        }
        _ => Err(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_args(args: &[&str]) -> (i32, String, String) {
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let code = run(&args, &mut out, &mut err);
        (
            code,
            String::from_utf8(out).unwrap(),
            String::from_utf8(err).unwrap(),
        )
    }

    fn file(dir: &tempfile::TempDir, name: &str, text: &str) -> String {
        let p = dir.path().join(name);
        fs::write(&p, text).unwrap();
        p.to_string_lossy().into_owned()
    }

    #[test]
    fn trace_prints_a_result_that_passes_check() {
        let dir = tempfile::tempdir().unwrap();
        let case = file(
            &dir,
            "t.json",
            r#"{"current_a":2,"length_mm":100,"width_mm":2,"thickness_um":35,"ambient_c":20,"convection_w_m2k":20}"#,
        );
        let (code, out, _) = run_args(&["trace", &case]);
        assert_eq!(code, 0);
        let result = file(&dir, "r.json", &out);
        let (code, out, _) = run_args(&["check", &result]);
        assert_eq!(code, 0, "{out}");
        assert!(out.contains("# 0 findings"));
    }

    #[test]
    fn network_prints_the_rise() {
        let dir = tempfile::tempdir().unwrap();
        let case = file(
            &dir,
            "n.json",
            r#"{"power_w":2,"ambient_c":25,"elements":[{"type":"resistance","k_per_w":5}]}"#,
        );
        let (code, out, _) = run_args(&["network", &case]);
        assert_eq!(code, 0);
        assert!(out.contains("\"value\": 10.0"), "{out}");
    }

    #[test]
    fn a_hot_case_exits_1_and_a_bad_file_exits_2() {
        let dir = tempfile::tempdir().unwrap();
        let hot = file(
            &dir,
            "h.json",
            r#"{"current_a":80,"length_mm":100,"width_mm":2,"thickness_um":35,"ambient_c":20,"convection_w_m2k":20}"#,
        );
        let (code, _, err) = run_args(&["trace", &hot]);
        assert_eq!(code, 1);
        assert!(err.contains("PHOENIX-THLA-002"), "{err}");
        let junk = file(&dir, "j.json", "{");
        assert_eq!(run_args(&["trace", &junk]).0, 2);
        assert_eq!(run_args(&["trace", "missing.json"]).0, 2);
        assert_eq!(run_args(&[]).0, 2);
    }

    #[test]
    fn check_lists_what_is_missing() {
        let dir = tempfile::tempdir().unwrap();
        let bare = file(
            &dir,
            "b.json",
            r#"{"quantity":"temperature_rise","value":12.0,"unit":"K","kind":"estimate"}"#,
        );
        let (code, out, _) = run_args(&["check", &bare]);
        assert_eq!(code, 1);
        assert!(
            out.contains("[hypotheses]") && out.contains("[validity]"),
            "{out}"
        );
    }

    #[test]
    fn the_settings_file_changes_the_limit() {
        let dir = tempfile::tempdir().unwrap();
        let case = file(
            &dir,
            "t.json",
            r#"{"current_a":2,"length_mm":100,"width_mm":2,"thickness_um":35,"ambient_c":20,"convection_w_m2k":20}"#,
        );
        let cfg = file(&dir, "c.cfg", "max_temperature_c = 30\n");
        let (code, _, err) = run_args(&["--config", &cfg, "trace", &case]);
        assert_eq!(code, 1, "{err}");
        let badcfg = file(&dir, "d.cfg", "nope = 1\n");
        assert_eq!(run_args(&["--config", &badcfg, "trace", &case]).0, 2);
    }
}
