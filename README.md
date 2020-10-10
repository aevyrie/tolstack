<p align="center">
  <img src="docs/logo.png" width="350">
  <br/><br/>
  <b>🚧 This application is in development, untested, unstable, and not ready for general use. 🚧</b>
  <br/>
</p>
<br/>

## Overview

TolStack is an open source tolerance analysis application for building and analyzing 1D geometric tolerance models. The goal of this tool is to help make tolerance analysis fast, easy, and error free. Built as a learning project with Rust using [`iced`](https://github.com/hecrj/iced).

[Read the TolStack user guide](https://aevyrie.github.io/tolstack/book/)

### Disclaimer

This software should only be used by engineers who are able to independently verify the correctness of its output. The software is provided as is, without warranty of any kind, inluding but not limited to the correctness of its output. The intent of this software is to aid you in the exploration tolerance analysis, not to replace existing methods of analysis or verification.

### Features

* Build one-dimensional tolerance stackups in a visual editor
* Evaluate and tune your tolerances with:
  * Monte Carlo analysis
  * RSS analysis
* Export results to CSV

### Screenshot

![Screenshot](docs/screenshot.png)

## Build Instructions

1. Install Rust via [Rustup](https://www.rust-lang.org/tools/install).
2. Clone the repository with `git clone https://github.com/aevyrie/tolstack.git`
3. From the `tolstack` directory, run `cargo run --release` to build and launch the application with compiler optimizations.

### Hardware and Software Requirements

* Note: make sure your graphics drivers are up to date!
* Linux/Windows: You will need a modern graphics card that supports Vulkan
  * Integrated graphics (Intel HDxxx) requires vulkan support, check [here](https://www.intel.com/content/www/us/en/support/articles/000005524/graphics.html)
* MacOS: the backend uses Metal, check [here](https://en.wikipedia.org/wiki/Metal_(API)#Supported_GPUs) for requirements

## License
This project is licensed under the [MIT license](https://github.com/aevyrie/tolstack/blob/master/LICENSE).

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in tolstack by you, shall be licensed as MIT, without any additional terms or conditions.
