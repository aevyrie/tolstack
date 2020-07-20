# tolstack

A cross-platform tolerance analysis application for building and analyzing one-dimensional geometric tolerance models. Targets Windows, MacOS, and Linux. Written in Rust, UI built with [iced](https://github.com/hecrj/iced). This is a learning project.

**Breaking changes in master**

![Screenshot](docs/screenshot.png)

## Build Instructions

1. Clone the repository.
2. Install Rust via [Rustup](https://www.rust-lang.org/tools/install).
3. From the root directory, run `cargo run --release` to build and launch the application with compiler optimizations.

## Features

Tolerance Stack Model

* `Linear`: Linear dimensions (point A to B with some tolerance)
* `Float`: Single hole/pin pair (a part with one pin located in another part with one hole, with some tolerance on each dia)
* Supports unequal bilateral tolerances (+0.2/-0.1)

Analysis

* Monte Carlo simulation
* ~~RSS, worst case~~

Output

* Auto saving
* Manually save/load JSON project files - WIP UI support partially complete
* ~~Export simulation output data to CSV for plotting~~ WIP UI support

## Todo

#### Short Term

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
- [ ] Implement ribbon or menu bar
  - [ ] Autosave toggle
  - [ ] Save/Open project
  - [ ] Export to CSV
  - [ ] Zoom multiplier (apply to all values by storing in stylesheet?)
  - [ ] Dark/light mode
- [X] Implement standardized styling
  - [X] Hot-reload via serde monitoring json file
- [ ] Implement save/load dialog
- [ ] Calculation result scrollable area showing results in progress and completed
  - [ ] Add export results button on these result entries
  - [ ] Save out results as CSV
  - [ ] Calculation progress bar
- [ ] Implement RSS tolerance analysis
- [ ] Implement worst case tolerance calculation

#### Long Term

- [ ] Implement concept of parts and joints - connections between parts
  - [ ] Allow joints to be in parallel, e.g. two or more pins connecting two parts
  - [ ] Parts can start and end a number of parallel joints
  - [ ] Implement butt joints, pos/neg determine butt direction
- [ ] Generate plots in GUI
- [ ] Tolerance stack visualization
- [ ] Make threading more intelligent, detect num_cpus, revisit perf
- [ ] Test coverage
- [ ] Go 2D?
