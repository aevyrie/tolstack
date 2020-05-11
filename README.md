# TolStack

TolStack is a cross platform tolerance analysis application made for building and analyzing one-dimensional geometric tolerance models. Works on Windows, MacOS, and Linux. **Breaking changes in master**

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
- [ ] Make computation function async
- [ ] Add RSS tolerance analysis
- [ ] Add worst case tolerance calculation
- [ ] Implement unequal bilateral tolerances in GUI
- [ ] Implement save/load dialog

#### Long Term

- [ ] Implement concept of part joints - connection between parts
  - [ ] Allow joints to be in parallel, e.g. a pair of holes/pins connecting two parts
  - [ ] Parts can start and end a number of parallel joints
  - [ ] Implement butt joints, pos/neg determine butt direction
- [ ] Generate plots in GUI
- [ ] Tolerance stack visualization
- [ ] Make threading more intelligent, detect num_cpus
- [ ] Test coverage
- [ ] Go 2D?
