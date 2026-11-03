# EH3D-THERMAL-LAB: scope

Status: **early code**. EH3D-THERMAL-LAB has a library and a command line; most of what this page lists is still planned. This page says what it is meant to do, what it will not do, and where it sits. Nothing has been run on real data.

## Purpose

EH3D-THERMAL-LAB is planned as a thermal simulation lab coupled to the physical design.

Thermal simulation laboratory coupled to the physical design, in three progressive stages.

## What it is planned to do

* Stage 1: analytic calculations, loss estimation and simplified thermal networks.
* Stage 2: more advanced models and coupling with the mechanical geometry.
* Stage 3: electro-thermal and electromagnetic co-simulation, much later.
* Take the losses computed in the circuit simulation from the simulation domain and apply them as heat sources on the real board and enclosure geometry, without exporting or importing files.
* Every result states its hypotheses, boundary conditions, materials, uncertainty, limitations, kind of result and range of validity.
* Heating of high-current traces and heavy copper (Joule effect), and stress from the mismatch of thermal expansion between board and chassis, marked as estimates until compared with a solver or a measurement.

## What it will not do

* No result is produced without its conditions: a bare number gives false confidence.
* No result is called validated before it has been compared with a measurement.
* A run that does not converge is a failed or inconclusive result, never a number.
* Not a safety system: it does not replace the thermal protection of the hardware.

## Planned shape

A library without a window, written in Rust, with a command-line interface and, where it applies, a kind of task for the server. Programs of the suite exchange work through project files or through the server, never by depending on each other.

The package is a single crate (a library and a command line, no window) with `trace`, `network` and `check`; a case and a result are JSON files. Heat sources from the simulation domain, the coupling with the geometry of the mechanical tool and the comparison with measurements are not written.

## Where it sits

It belongs to the PHOENIX family of the EH3D Fabrication Suite: independent repositories that relate through versioned contracts. The design tool (EH3D-PHOENIX-PCB) does not depend at build time on the repositories that depend on it.
See DEPENDENCIES.md for the relations, REQUIREMENTS.md for the requirements and STATUS.md for the honest state.
