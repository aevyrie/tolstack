# TolStack

TolStack is a simple tolerance analysis application with a UI for building and analyzing one-dimensional tolerance models. **Breaking changes in master**

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
