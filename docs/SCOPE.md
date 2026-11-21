# EH3D-THERMAL-LAB: scope

Status: **early code**. EH3D-THERMAL-LAB is a library and a command line that estimate the temperature rise of a copper trace that carries a current and across a network of thermal resistances. It has been compared with closed-form values only. This page says what it is for, what it will not do and where it sits among the other programs of the suite.

## Purpose

The thermal laboratory of the PHOENIX family, coupled to the physical design in three stages: analytic estimates and simplified networks first, then more advanced models coupled to the mechanical geometry, and much later the electro-thermal co-simulation. Every result states its hypotheses, boundary conditions, materials, uncertainty, limitations, kind of result and range of validity.

## What it does

* Computes the temperature rise of a copper trace that carries a current (the Joule effect, with the resistance growing with the temperature and the cooling by convection) and checks it against the closed-form value when the air is at the reference temperature.
* Computes the rise across a network of thermal resistances in series and parallel (conduction, convection or a given value), in steady state and one dimension.
* Refuses a result that lacks its hypotheses, boundary conditions, materials, uncertainty, limitations, unit or range of validity, and refuses an inverted range.
* Gives an error with a stable code and no value for a case that gets hotter than the range of validity (thermal runaway included) or runs out of steps.
* Reads a case and a result as JSON and takes the limit from a settings file.
* Compares the model with a simulated thermocouple on the trace along a ramp of current, noticing an offset, a drift, a sticking or saturating sensor and noise, and ends the ramp at the first current the model refuses.

## What it will not do

* It produces no result without its conditions: a bare number gives false confidence.
* It calls no result validated before it has been compared with a measurement.
* A run that does not converge is a failed or inconclusive result, never a number.
* It is not a safety system and does not replace the thermal protection of the hardware.
* It does not take the losses of the simulation domain, couple with the geometry or compare with a measurement of a real board yet (see `REQUIREMENTS.md`).

## Where it sits

* **It reads:** a case (JSON), a result to check (JSON) and a settings file (the limit of temperature, the steps and the uncertainty that is written in a result).
* **It writes:** the result and the findings it is asked to print, to the standard output; it never writes to the files it reads.
* **It is read by:** nothing reads it yet: the design tool will read the estimates once the losses and the geometry are connected.
* **It depends on:** nothing of the suite at build time; the losses and the geometry of the board come from EH3D-PHOENIX-PCB and the enclosure from EH3D-PHOENIX-CAD.

## What it never decides

Whether a board or an enclosure is safe against heat, and the thermal protection, the limits, the interlocks and the emergency stop of any machine, belong to the hardware and to the person who designs it. The uncertainty written in a result is a placeholder from the settings until a measurement characterises it. Nothing here starts motion.

The requirements of the program, with the state of each and the test that shows it, are in `REQUIREMENTS.md`; the state in a paragraph is in `STATUS.md`.
