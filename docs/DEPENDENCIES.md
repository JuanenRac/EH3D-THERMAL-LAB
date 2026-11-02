# EH3D-THERMAL-LAB: dependencies

One row per relation with another repository. The kind is conceptual, compile-time or run-time. The state of a contract is: not written, drafted, accepted, frozen or published. A repository existing does not mean that its contract is ready: this project does not implement or consume a contract that is not accepted.

| From | To | Kind | Contract | Contract state | Minimum version | Tested version | Compatibility test | Change owner | What it blocks | Migration |
|---|---|---|---|---|---|---|---|---|---|---|
| EH3D-THERMAL-LAB | EH3D-PHOENIX-PCB | compile | simulation and thermal domains of the project | drafted | not set | not set | to be written with the first code | the owner of the contract | all of it | to be defined |
| EH3D-THERMAL-LAB | EH3D-PHOENIX-CAD | conceptual | geometry for stage 2 | not written | not set | not set | to be written with the first code | the owner of the contract | stage 2 | to be defined |
| EH3D-THERMAL-LAB | EH3D-COMMON | compile | container, diagnostics | drafted | not set | not set | to be written with the first code | the owner of the contract | all of it | to be defined |
| EH3D-THERMAL-LAB | EH3D-LAB-BRIDGE | conceptual | measurements for comparison | not written | not set | not set | to be written with the first code | the owner of the contract | validation | to be defined |
| EH3D-THERMAL-LAB | EH3D-COMPONENT-INTELLIGENCE | conceptual | thermal limits of components | not written | not set | not set | to be written with the first code | the owner of the contract | nothing here is blocked by it | to be defined |

Rules: repositories are built next to each other as sibling folders and each declares in its manifest the versions of the others that it needs, once those exist. A lower layer never depends on a higher one. The manifest field for build dependencies stays empty until code exists.
