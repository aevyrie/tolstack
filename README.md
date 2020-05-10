# TolStack

Simple 1D tolerance analysis tool. **Unfinished and Unstable WIP.**

![Screenshot](docs/screenshot.png)

## Build Instructions

1. Clone the repository.
2. Install Rust via [Rustup](https://www.rust-lang.org/tools/install).
3. From the root directory, run `cargo run --release` to build and launch the application.

## Features

Mechanical Interfaces

* `Linear`: Linear dimensions (point A to B)
* `Float`: Single hole/pin pair (a part with one pin located in another part with one hole)
* `Compound`: Double hole/pin pairs in parallel (a part with two pins located in another part with two holes)

Tolerances

* Unilateral tolerances (+0/-0.5)
* Equal bilateral tolerances (+/-0.5)
* Unequal bilateral tolerances (+0.2/-0.1)

Input/Output

* Auto saving - no manual save/load yet
* ~~Saving/loading JSON files~~
* ~~Exports simulation output data to CSV for plotting~~

## Todo

* GUI for building tolerance model as well as saving/loading.
* Generalize the Compound tolerance interface to instead allow any tolerance to be in parallel instead of serial only.
* Add RSS tolerance analysis
* Add worst case tolerance calculation
* Generate plots in GUI
* Tolerance stack visualization
* Make threading more intelligent, detect num_cpus
* Test coverage
* Revisit optimization
* Go 2D?

#### Parallel Tolerance Notes

* Linear - need to define whether to take the minimum or maximum of the parallel dimensions as the result
  * How to refer to this and display this to user?
* Float - much simpler
* combinations? need to be able to define linear/float 
