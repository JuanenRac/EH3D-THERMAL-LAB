# EH3D-THERMAL-LAB: requirements

This register is the single source of the state of each capability. Status values: planned, prototype, implemented, validated. 4 requirements are `prototype` (code and tests exist, only on data made by the tests) and 3 are `planned` (THLA-004, THLA-005, THLA-006); none has been used on real data or validated. High test coverage is not proof of geometric or electrical correctness, so each requirement says how it will be checked and what result is expected.

| Requirement | Description | Status | How it will be checked | Expected result | Known limits |
|---|---|---|---|---|---|
| THLA-001 | Analytic stage-1 estimates with stated conditions. | prototype | automatic: cases with analytic solutions | Result within the stated tolerance of the analytic value. | Only a copper trace with convection and a series and parallel network; checked against closed-form values, never against a solver or a measurement. The uncertainty written in the result is a placeholder from the settings. |
| THLA-002 | Refuse a result that lacks hypotheses, boundary conditions or limits. | prototype | automatic: negative tests | Such a result cannot be created. | Checks that the lists, the unit, the uncertainty and the range exist and are usable; it cannot know whether what they say is true. |
| THLA-003 | Simplified thermal network. | prototype | automatic | Network results agree with the analytic cases. | Series and parallel resistances of conduction, convection or a given value, in steady state and one dimension; checked against the closed-form slab and parallel cases. |
| THLA-004 | Apply losses from the simulation domain as heat sources. | planned | automatic | Energy balance holds within the stated tolerance. | Not started. |
| THLA-005 | Couple with the geometry of the mechanical tool. | planned | automatic plus review | Geometry changes update the model. | Not started. |
| THLA-006 | Compare with measurements through the laboratory bridge. | planned | physical measurement | Deviation reported with uncertainty. | Not started. |
| THLA-007 | Non-converged runs are failed or inconclusive. | prototype | automatic | No numeric result from a non-converged run. | A case that gets hotter than the range of validity (this includes thermal runaway) or runs out of steps gives an error with a stable code and no value. |

Rules: a claim of correctness names the corpus, the tolerance and the tool version it holds for. A phase is complete when the measured rate does not drop, every failure is classified and fixed or documented, and the measured figure is reported.
