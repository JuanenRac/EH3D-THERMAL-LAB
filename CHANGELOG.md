# Changelog

All notable changes to this project are documented here.

## [Unreleased]

- **No result without a number:** a network with extreme sizes printed a result whose value was empty, with exit code 0. It is now refused as outside the range of validity.
- **A JSON array is no longer read as a case or a result:** the fields were taken by position, with no names to check. Only an object is read.
- **1 more test** of the Rust code, for the above.

## [0.0.4] - Simulated bench, requirement register and many more tests

- **More tests, for the shared tools of the base and for the thermal code:** `tools/test_shared_tools.py` adds 65 tests that run the lessons linter, the workflow check, the CHANGELOG check of a pull request and the banner check on small inputs made by the test itself (findings, baselines, exit codes and the shape of what they print). The Rust code gets 22 more: the rise of a trace does not depend on its length, a wider or thicker trace rises less, the same case gives the same bytes every time, an ambient below zero or at the highest temperature is out of validity, numbers at the extremes (1e-300 to the largest float) give an error or a finite rise and never a panic, the result of the settings carries their uncertainty and range, a text that is not a result is an error, every missing condition and every unusable range is named, the judge of the simulated bench stops exactly on the edge of the tolerance, and the seeded generator repeats for the same seed.

- **The threat model says what the code already does:** the rows of `docs/THREAT_MODEL.md` that have a mitigation in the code and a test for it are marked `partial` and name the tests and what is still not covered; the rows without either stay `planned`.

- **A simulated bench (`sim`):** a thermocouple on the trace made of the model itself with an offset, noise, a drift, dropouts, a sensor that has not connected yet, one that sticks and one that saturates, and a ramp of the current that ends at the first current the model refuses, with a verdict for each step (the reading agrees with the uncertainty of the result, the model says too little or too much, no reading, no number from the model). It lets the comparison with a reading be tried on every kind of bad reading before there is a bench; the plant is the model, so it says nothing about how good the model is. Eight new tests (29 in all) with fixed seeds cover each fault and 30 seeded campaigns. Nothing was measured.
- **The register of requirements names a test for each row:** `docs/REQUIREMENTS.md` has one row for each capability with an identifier (`THLA-001` on), its state (planned, prototype, implemented or validated), how far it is (done, partial, pending or blocked), its evidence (simulator, host-test or none), the test that shows it and its limits; a capability that is not written names the test that will show it. `tests/reqs.rs` reads the table and fails when a test does not exist, when a state does not agree with the progress or the evidence, when `docs/STATUS.md` does not say the same numbers, when a README does not link the documents or when a document holds a date or a private reference. `docs/SCOPE.md`, `docs/STATUS.md` and `docs/NOT_YET.md` are rewritten to the same shape and the eight READMEs link the five documents.
- The shared workflow checker (`tools/workflow_hygiene.py`) is the current one of the documentation hub: the filters under a trigger (branches, paths, types) are no longer read as events.

## [0.0.3] - Shared validation checks, split CI workflows and pinned Rust toolchain

- **The shared validation tools are updated:** the validator also runs the workflow and lessons checks of the suite with their latest rules, including a check to run before every push.
- **The CI validation checks the workflows and the lessons of past CI failures against a baseline:** `tools/workflow_hygiene.py` and `tools/lessons_lint.py` (copies of the shared tools) run in the validation, and only findings that are not in `tools/workflow_hygiene_baseline.json` and `tools/lessons_lint_baseline.json` fail it, so what exists today stays as it is and a new one is caught. `python tools/workflow_hygiene.py --root . --detail` and `python tools/lessons_lint.py --repo .` list the findings.
- **The validation of the CI is the reusable one of the documentation hub:** the job `validate` calls `validate.yml` of EH3D-DOCS (pinned by commit) and the check of the CHANGELOG is a job of its own.
- **The CI is split in two workflows:** the validation of the repository (manifest, documentation, pictures, tools) stays in `ci.yml` and the Rust jobs move to `rust.yml`, which does not run for a change that only touches Markdown files or pictures, so a change of the documentation no longer builds the code on two systems, and they keep a cache of the build. The names of the jobs are the same.
- **The Rust toolchain is pinned:** `rust-toolchain.toml` fixes the version of Rust (the same one in every repository of the suite) with `rustfmt` and `clippy`, the workflows install that version and not `stable`, so a new release of Rust cannot turn the CI red by itself.

## [0.0.2] - First release: the library, the command line, the stable error codes and the tests

- Add the library: the temperature rise of a copper trace with a current (Joule effect with temperature-dependent resistance, cooling by convection) and across a network of thermal resistances in series and parallel.
- Refuse a result that lacks hypotheses, boundary conditions, materials, uncertainty, limitations or range of validity, and give no number for a case out of range or without convergence, with stable codes `PHOENIX-THLA-001` to `PHOENIX-THLA-004`.
- Add the command line (`trace`, `network`, `check`) and a settings file for the material values, the range and the iteration; run format, lints and tests of the Rust code on Linux and Windows in the CI.
- **Brand:** Add the application icon and the logos of the program.
- **Shared checks updated:** the validator also compares the badge and the Version row of the README with the manifest, fails when one of its checks cannot run, and the CHANGELOG step of the workflow reads the base branch from the environment.
- **The workflows are hardened:** every action is pinned by its commit with its version in a comment, and the checkouts keep no credentials.

## [0.0.1] - The base of the repository

- **The base that every repository of the suite has:** the manifest, the README files in eight languages, this changelog, the CI workflow, the issue and pull-request templates, and the shared checks of EH3D-DOCS (the repository check and the drawers of the two README pictures).
- **A planned project, described honestly:** the scope, the requirements (all planned), the relations with other repositories, the threat model, the decisions that apply and the rules for writing code are in `docs/`; there is no code yet.
