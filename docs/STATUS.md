# EH3D-THERMAL-LAB: status

## Current state

EH3D-THERMAL-LAB is early code on a computer: a Rust library and a command line (`trace`, `network`, `check`) that compute the temperature rise of a copper trace that carries a current and across a network of thermal resistances in series and parallel, and that refuse a result without its hypotheses, boundary conditions, materials, uncertainty, limitations and range of validity. A simulated thermocouple on the trace, with a current ramp, checks that the model and the sensor are told apart. They have been compared with closed-form values only; nothing has been compared with a solver or a measurement and no requirement is validated.

## Capabilities

The official state of each capability is in `REQUIREMENTS.md`. The register has 27 requirements: 21 implemented, 3 prototype, 3 planned and 0 validated. By progress: 21 done, 3 partial, 0 pending and 3 blocked. This page never states a state that `REQUIREMENTS.md` does not.

## Open questions

* Which solver, if any, will be used from stage 2 on.
* How the uncertainty of an estimate is characterised once a bench exists.

## What it needs from other repositories

* EH3D-PHOENIX-PCB: the losses and the geometry of the board.
* EH3D-PHOENIX-CAD: the geometry of the enclosure.
* EH3D-LAB-BRIDGE: the comparison with a measurement.

## Known technical debt and omitted tests

* 62 tests of the Rust code (nine of them keep this register true) and the 68 of the base (65 of them test the shared tools of the base); the Rust tests compare with closed-form values and never touch real data.
* Eight of the tests run the simulated thermocouple (`src/lib.rs`): offset, noise, drift, dropouts, a sensor that sticks or saturates, and a current ramp that ends where the model refuses.
* Not tested: a comparison with a solver or with a measurement.
* `cargo deny` is not run yet.
