# TolStack

This is a cross platform tolerance analysis application made for building and analyzing one-dimensional geometric tolerance models. Works on Windows, MacOS, and Linux.

**Breaking changes in master**

![Screenshot](docs/screenshot.png)

## Build Instructions

1. Clone the repository.
2. Install Rust via [Rustup](https://www.rust-lang.org/tools/install).
3. From the root directory, run `cargo run --release` to build and launch the application with compiler optimizations.

## Features

Tolerances

* `Linear`: Linear dimensions (point A to B with some tolerance)
* `Float`: Single hole/pin pair (a part with one pin located in another part with one hole, with some tolerance on each dia)
* ~~`Compound`: Double hole/pin pairs in parallel (a part with two pins located in another part with two holes)~~ Deprecated

Tolerances

* ~~Unilateral tolerances (+0/-0.5)~~ Need UI support
* Equal bilateral tolerances (+/-0.5)
* ~~Unequal bilateral tolerances (+0.2/-0.1)~~ Need UI support

Input/Output

* Auto saving
* ~~Manually save/load JSON project files~~ Need UI support
* ~~Export simulation output data to CSV for plotting~~ Need UI support

## Todo

#### Short Term

- [x] GUI for building tolerance model
- [X] Make computation function async
- [X] Add simulation parameter controls
- [X] Refactor code structure to make extension easier
  - [X] Break out UI widgets into using own internal structure to greatly simplify the top `view()` function
  - [X] Consolidate/simplify ToleranceType matching
  - [X] Remove redundant SimulationResult, use result in the SimulationState
- [ ] Implement unequal bilateral tolerances in GUI
- [ ] Add sigma input on tolerance entries
- [ ] Add hole/pin diameter input on `Float` entries
- [ ] Show tolerance summary on entries in idle state
- [ ] Implement standardized styling
  - [ ] Investigate hot-reload via serde monitoring json file, enable during debug
- [ ] Implement ribbon or menu bar
  - [ ] Autosave toggle
  - [ ] Save/Open project
  - [ ] Export to CSV
  - [ ] Text size increment
  - [ ] Dark/light mode
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
