# Changelog

All notable changes to this project are documented here.

## [Unreleased]
- Add the library: the temperature rise of a copper trace with a current (Joule effect with temperature-dependent resistance, cooling by convection) and across a network of thermal resistances in series and parallel.
- Refuse a result that lacks hypotheses, boundary conditions, materials, uncertainty, limitations or range of validity, and give no number for a case out of range or without convergence, with stable codes `PHOENIX-THLA-001` to `PHOENIX-THLA-004`.
- Add the command line (`trace`, `network`, `check`) and a settings file for the material values, the range and the iteration; run format, lints and tests of the Rust code on Linux and Windows in the CI.
- **Brand:** Add the application icon and the logos of the program.
- **Shared checks updated:** the validator also compares the badge and the Version row of the README with the manifest, fails when one of its checks cannot run, and the CHANGELOG step of the workflow reads the base branch from the environment.
- **The workflows are hardened:** every action is pinned by its commit with its version in a comment, and the checkouts keep no credentials.

## [0.0.1] - The base of the repository

- **The base that every repository of the suite has:** the manifest, the README files in eight languages, this changelog, the CI workflow, the issue and pull-request templates, and the shared checks of EH3D-DOCS (the repository check and the drawers of the two README pictures).
- **A planned project, described honestly:** the scope, the requirements (all planned), the relations with other repositories, the threat model, the decisions that apply and the rules for writing code are in `docs/`; there is no code yet.
