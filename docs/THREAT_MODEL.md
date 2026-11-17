# EH3D-THERMAL-LAB: threat model

Basic model for early code. The rows marked partial have a mitigation written and tests for it, run only on data made by the tests themselves; the rows marked planned have nothing yet. No row is assessed against real data.

| Threat | Planned mitigation | Planned test | State | Residual risk |
|---|---|---|---|---|
| A thermal result trusted outside its range of validity | a result must name its conditions and a usable range, a hot source gives no number, and the ramp ends at the first current the model refuses | `each_missing_condition_is_reported_with_the_code`, `an_inverted_range_is_not_usable`, `a_hot_source_gives_no_number` | partial | the range is the one the case declares; no real board has been measured |
| Resource exhaustion by a large model | limits on size, time and memory | limit tests | planned | not assessed: nothing is implemented |
| Corrupt or malicious project or input files (oversized entries, path tricks, deep nesting, resource exhaustion) | bad networks and bad inputs are refused and the JSON reader refuses unknown fields | `bad_networks_are_refused`, `bad_inputs_are_refused`, `json_round_trip_and_unknown_field` | partial | hostile samples written by hand; no fuzzing yet |
| Temporary files, logs and reports that reveal private designs | no private data in outputs; redaction | automated pattern search | planned | not assessed: nothing is implemented |
| Compromised dependency or binary | pinned versions, licence and advisory checks, software bill of materials for releases | CI checks | planned | not assessed: nothing is implemented |
| Power loss or forced close during a save | atomic write: temporary file, then rename; old file never deleted first | interruption tests at controlled points | planned | not assessed: nothing is implemented |

Minimum negative tests once there is code: hostile files and resource limits; secrets absent from logs, errors and reports; pinned dependencies and licence review before publishing. Report a vulnerability privately as described in SECURITY.md, without attaching private designs.
