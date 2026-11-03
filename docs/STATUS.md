# EH3D-THERMAL-LAB: status

## Current state

EH3D-THERMAL-LAB is early code. Version 0.0.1 holds the base that every repository of the suite carries (manifest, README files in eight languages, changelog, CI, shared checks) and the written scope, requirements, dependencies, threat model and decisions. It also has a Rust library and a command line that compute the temperature rise of a copper trace that carries a current (Joule effect, resistance growing with the temperature, cooling by convection) and across a network of thermal resistances in series and parallel, and that refuse a result without its hypotheses, boundary conditions, materials, uncertainty, limitations and range of validity. They have run only on data made by their own tests. Nothing has been run on a machine with real data, and no requirement is validated.

## Capabilities

The official state of each capability is in REQUIREMENTS.md: 7 requirements: 4 `prototype` and 3 `planned` (THLA-004, THLA-005, THLA-006). This page never states a state that REQUIREMENTS.md does not.

## Open questions

* Which solver, if any, will be used from stage 2 on.

## Needs from other repositories

* EH3D-PHOENIX-PCB: losses and geometry of the board
* EH3D-PHOENIX-CAD: enclosure geometry

## Known technical debt and omitted tests

21 tests of the Rust code and the 3 of the base; the Rust tests compare with closed-form values and never touch real data. Not tested: a comparison with a solver or a measurement. `cargo deny` is not run yet.
