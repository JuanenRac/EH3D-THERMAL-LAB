# EH3D-THERMAL-LAB: requirements

This register is the single source of the state of each capability. Status values: planned, prototype, implemented, validated. Every requirement is `planned` today: nothing is implemented or verified. High test coverage is not proof of geometric or electrical correctness, so each requirement says how it will be checked and what result is expected.

| Requirement | Description | Status | How it will be checked | Expected result | Known limits |
|---|---|---|---|---|---|
| THLA-001 | Analytic stage-1 estimates with stated conditions. | planned | automatic: cases with analytic solutions | Result within the stated tolerance of the analytic value. | Not started: no limits known yet. |
| THLA-002 | Refuse a result that lacks hypotheses, boundary conditions or limits. | planned | automatic: negative tests | Such a result cannot be created. | Not started: no limits known yet. |
| THLA-003 | Simplified thermal network. | planned | automatic | Network results agree with the analytic cases. | Not started: no limits known yet. |
| THLA-004 | Apply losses from the simulation domain as heat sources. | planned | automatic | Energy balance holds within the stated tolerance. | Not started: no limits known yet. |
| THLA-005 | Couple with the geometry of the mechanical tool. | planned | automatic plus review | Geometry changes update the model. | Not started: no limits known yet. |
| THLA-006 | Compare with measurements through the laboratory bridge. | planned | physical measurement | Deviation reported with uncertainty. | Not started: no limits known yet. |
| THLA-007 | Non-converged runs are failed or inconclusive. | planned | automatic | No numeric result from a non-converged run. | Not started: no limits known yet. |

Rules: a claim of correctness names the corpus, the tolerance and the tool version it holds for. A phase is complete when the measured rate does not drop, every failure is classified and fixed or documented, and the measured figure is reported.
