<p align="center">
  <img src="docs/logo.png" width="498">
</p>
<br/>

Tolstack is a tolerance analysis application for building and analyzing 1D geometric tolerance models. The goal of this tool is to make tolerance stackup analysis fast, easy, and error free. Built as a learning project with Rust using [`iced`](https://github.com/hecrj/iced).

**This application is in development and not ready for use.**

## Overview

![Screenshot](docs/screenshot.png)

[Tolerance analysis](https://en.wikipedia.org/wiki/Tolerance_analysis) is used in Mechanical Engineering to quantify the accumulated dimensional variation in assemblies of parts. This is used to define part tolerances, and later verify that manufacturer processes are statistically capable of producing parts to this tolerance spec. Generally, the goal is to specify the widest possible tolerances to minimize scrap ($$$) while ensuring any combination of parts within these tolerances still fit together and function. GD&T (ASME Y14.5) is commonly used as the languge to express three-dimensional tolerances.

### 1D Tolerance Analysis

This application does not attempt to model all of the tolerances in your assembly, rather, this is a tool to help you model and understand critical tolerance stacks in one dimension. This greatly simplifies the modelling process and generally makes for much clearer, actionable, output. To construct a 1D model, you will need to:

1. Define your target measurement - for what measurement do you need to analyze the tolerance stackup?
2. Define an axis to project your dimensions and tolerances onto (often times you can just project onto a plane).
3. Define the positive and negative directions along your axis - this is very important!
4. Determine the chain of dimensions needed to define the stackup that results in your targer measurement.
5. Using this chain of dimensions, record the dimensions and tolerances as projected on your axis, making sure the signs are correct.

## Build Instructions

1. Clone the repository.
2. Install Rust via [Rustup](https://www.rust-lang.org/tools/install).
3. From the root directory, run `cargo run --release` to build and launch the application with compiler optimizations.

## Features

### Tolerance Stack Model

* `Linear`: Linear dimensions (point A to B with some tolerance)
* `Float`: Represents a connection between parts with a hole/pin pair
* Supports unequal bilateral tolerances (+0.2/-0.1)

### Analysis

* Monte Carlo simulation
* Worst case stackup
* WIP: ~~RSS~~
* WIP: ~~Hole-pin connections in parallel~~

### Output

* Auto saving
* WIP: ~~Manually save/load JSON project files~~
* WIP: ~~Export results to CSV~~

## Roadmap

### v0.1

- [x] GUI for building tolerance model
- [X] Make computation function async
- [X] Add simulation parameter controls
- [X] Refactor code structure to make extension easier
  - [X] Break out UI widgets into using own internal structure to greatly simplify the top `view()` function
  - [X] Consolidate/simplify ToleranceType matching
  - [X] Remove redundant SimulationResult, use result in the SimulationState
- [X] Implement unequal bilateral tolerances in GUI
- [X] Add sigma input on tolerance entries
- [X] Add hole/pin diameter input on `Float` entries
- [X] Show tolerance summary on entries in idle state
- [ ] Ribbon menu bar
  - [ ] Autosave toggle
  - [X] Save/Open project
  - [ ] Save As project
  - [ ] Export calculation data to CSV
  - [ ] Button icons
- [X] Stylesheets
  - [X] Hot-reload via serde monitoring json file
- [ ] Analysis features
  - [ ] RSS tolerance analysis
  - [X] Worst case tolerance calculation
- [ ] Unit tests
  - [ ] Basic tolerance stack with linear + float, verify output

### v0.2

- [ ] Undo/Redo
- [ ] Calculation results history
  - [ ] Scrollable calculation result area
  - [ ] Add export results button on the result entries
  - [ ] Calculation progress
  - [ ] Serialize calculation results with save
- [ ] Tolerance model improvements
  - [ ] Add distribution options
    - [ ] `Linear`: Normal, Normal Clipped (OOS rejected), Flat
    - [ ] `Float` : Normal, Normal Clipped (OOS rejected), Flat
- [ ] Appearance
  - [ ] Zoom multiplier (apply to all values by storing in stylesheet?)
  - [ ] Dark/light mode

### Unscheduled

- [ ] Add unit selection (mm/in) on a per-tolerance and project basis
- [ ] Compute per-measurment sensitivity, display as absolute or percentage of total
- [ ] Compare calculation results side-by-side showing the full stack
- [ ] Sidebar with multiple tolstacks in a project
- [ ] Float interference control
- [ ] Report number of `Float` tolerances that result in a diametric interference
- [ ] `Float` : Reject connections with more than n units of interference

#### Long Term

- [ ] Implement concept of parts and joints - connections between parts
  - [ ] Allow joints to be in parallel, e.g. two or more pins connecting two parts
  - [ ] Parts can start and end a number of parallel joints
- [ ] Generate plots in GUI
- [ ] Tolerance stack visualization
- [ ] Make threading more intelligent, detect num_cpus, revisit perf, thread pooling?
- [ ] Go 2D?
- [ ] Constraint solver?
