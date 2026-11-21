# EH3D-THERMAL-LAB: requirements

This register is the single source of the state of each capability: `STATUS.md` and the README files cite it and never state a state that it does not. A state is one of `planned`, `prototype`, `implemented` and `validated`. `validated` means that a person has run the capability on a real machine and the evidence of the manifest says `bench` or `field`; nothing here is validated, because nothing of this program has been compared with a solver or with a measurement on a real board.

The register has 27 requirements: 21 implemented, 3 prototype, 3 planned and 0 validated. By progress: 21 done, 3 partial, 0 pending and 3 blocked. High coverage is not proof of correctness, so each row says how it is checked: the test that names it, and what it shows.

## How to read the register

* **ID:** `THLA-NNN`, never reused and never renumbered; a new capability takes the next number.
* **State:** `implemented` is code that is complete and tested on a computer, `prototype` is part of the capability, `planned` is not written.
* **Progress:** `done`, `partial`, `pending` (can be written without hardware) or `blocked` (waits for hardware or for another program of the suite).
* **Evidence:** `simulator` when the test runs against a simulated thermocouple on the trace, with a current ramp, whose faults are injected on purpose, `host-test` when it checks a calculation, a reader or a screen-less piece, `none` when nothing is written yet.
* **Test:** the acceptance criterion: `file::name` is the test `name` of `tests/file.rs` (`src/file::name` is a test inside `src/file.rs`), which must exist. For a capability that is not written the cell says `planned:` and the name of the test that will show it; the day that test exists, the row must change.
* **Limits:** what is not done or not proven; a row that is `partial` or `blocked` always says it.

The check is `tests/reqs.rs`: it reads this table, finds each test and fails when a state, a test or a count does not agree.

## The estimates

| ID | Requirement | State | Progress | Evidence | Test | Limits |
|---|---|---|---|---|---|---|
| THLA-001 | Analytic stage-1 estimate of a trace with the conditions stated. | prototype | partial | host-test | `src/trace::matches_the_closed_form_when_the_air_is_at_the_reference_temperature` | Only a copper trace with convection; it is checked against closed-form values, never against a solver or a measurement, and the uncertainty written in the result is a placeholder from the settings. |
| THLA-002 | Refuse a result that lacks hypotheses, boundary conditions or limits. | prototype | partial | host-test | `src/result::each_missing_condition_is_reported_with_the_code` | It checks that the lists, the unit, the uncertainty and the range exist and are usable; it cannot know whether what they say is true. |
| THLA-003 | Simplified thermal network of series and parallel resistances. | prototype | partial | host-test | `src/network::a_slab_matches_the_closed_form` | Steady state and one dimension, checked against the closed-form slab and parallel cases. |
| THLA-004 | Apply the losses of the simulation domain as heat sources. | planned | blocked | none | `planned:heat_sources::the_losses_of_the_simulation_keep_the_energy_balance` | Waits for the simulation domain to give the losses of the board. |
| THLA-005 | Couple with the geometry of the mechanical tool. | planned | blocked | none | `planned:geometry::a_change_of_the_geometry_updates_the_model` | Waits for the geometry of the board and of the enclosure from the design tools. |
| THLA-006 | Compare with measurements through the laboratory bridge. | planned | blocked | none | `planned:measurement::a_measured_curve_is_compared_with_the_estimate_with_its_uncertainty` | Needs a real bench and the laboratory bridge; nothing has been measured. |
| THLA-007 | A run that does not converge is an error with a stable code, never a number. | implemented | done | host-test | `src/trace::running_out_of_steps_is_not_converged` | - |
| THLA-008 | A case hotter than the range of validity gives no number. | implemented | done | host-test | `src/trace::a_hot_case_gives_no_number` | - |
| THLA-009 | More current gives more rise and a better cooling gives less. | implemented | done | host-test | `src/trace::more_current_gives_more_rise_and_a_better_cooling_gives_less` | - |
| THLA-010 | Bad inputs are refused. | implemented | done | host-test | `src/trace::bad_inputs_are_refused` | - |
| THLA-011 | Series resistances add and equal branches in parallel halve. | implemented | done | host-test | `src/network::series_add_and_equal_branches_in_parallel_halve` | - |
| THLA-012 | A hot source in a network gives no number. | implemented | done | host-test | `src/network::a_hot_source_gives_no_number` | - |
| THLA-013 | Bad networks are refused. | implemented | done | host-test | `src/network::bad_networks_are_refused` | - |
| THLA-014 | An inverted range of validity is not usable. | implemented | done | host-test | `src/result::an_inverted_range_is_not_usable` | - |
| THLA-015 | A result makes a JSON round trip and an unknown field is refused. | implemented | done | host-test | `src/result::json_round_trip_and_unknown_field` | - |

## The command line and the settings

| ID | Requirement | State | Progress | Evidence | Test | Limits |
|---|---|---|---|---|---|---|
| THLA-016 | `trace` prints a result that passes `check`. | implemented | done | host-test | `src/cli::trace_prints_a_result_that_passes_check` | - |
| THLA-017 | `network` prints the rise. | implemented | done | host-test | `src/cli::network_prints_the_rise` | - |
| THLA-018 | A hot case exits with 1 and a bad file exits with 2. | implemented | done | host-test | `src/cli::a_hot_case_exits_1_and_a_bad_file_exits_2` | - |
| THLA-019 | `check` lists what is missing. | implemented | done | host-test | `src/cli::check_lists_what_is_missing` | - |
| THLA-020 | The settings file changes the limit. | implemented | done | host-test | `src/cli::the_settings_file_changes_the_limit` | - |
| THLA-021 | A bad line of the settings file is refused. | implemented | done | host-test | `src/config::bad_lines_are_refused` | - |

## Simulated thermocouple on the trace

| ID | Requirement | State | Progress | Evidence | Test | Limits |
|---|---|---|---|---|---|---|
| THLA-022 | A perfect sensor agrees with the model at every step of the ramp. | implemented | done | simulator | `src/lib::a_perfect_sensor_agrees_with_the_model_at_every_step` | The sensor is simulated; no thermocouple has been read. |
| THLA-023 | The ramp ends at the first current the model refuses and the sensor is not read there. | implemented | done | simulator | `src/lib::the_ramp_ends_at_the_first_current_the_model_refuses_and_the_sensor_is_not_read_there` | - |
| THLA-024 | An offset in either direction is noticed as the model being low or high. | implemented | done | simulator | `src/lib::an_offset_in_either_direction_is_noticed_as_the_model_being_low_or_high` | - |
| THLA-025 | A sensor that sticks or saturates is caught. | implemented | done | simulator | `src/lib::a_sensor_that_sticks_is_caught_once_the_model_has_moved_away_from_it` | - |
| THLA-026 | A drift is noticed late and a dead sensor that comes alive recovers. | implemented | done | simulator | `src/lib::a_drift_is_noticed_late_and_a_dead_sensor_that_comes_alive_recovers` | - |
| THLA-027 | A seeded campaign of noise and dropouts never calls a good reading bad and repeats exactly. | implemented | done | simulator | `src/lib::a_seeded_campaign_of_noise_and_dropouts_never_calls_a_good_reading_bad_and_repeats_exactly` | - |
