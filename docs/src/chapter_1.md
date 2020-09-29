# Introduction

## Overview

TolStack is a tolerance analysis application for building and analyzing 1D geometric tolerance models. The goal of this tool is to make tolerance stackup analysis fast, easy, and error free. Built as a learning project with Rust using [`iced`](https://github.com/hecrj/iced).

### Use Cases

One-dimensional tolerance analysis is useful for answering questions like:
* Will this stack of plastic parts and PCBs fit into my enclosure 99.99% of the time?
* When I depress this button, will I make electrical contact with the switch before it bottoms out?
* Knowing the given tolerances of my purchased components, what part tolerances do I need to function?

### Features

* Build one-dimensional tolerance stackups in a visual editor
* Evaluate and tune your tolerances with:
  * Monte Carlo analysis
  * RSS analysis
  * Worst case tolerance analysis
* Export results to CSV

## Background

[Tolerance analysis](https://en.wikipedia.org/wiki/Tolerance_analysis) is used in Mechanical Engineering to quantify the accumulated dimensional variation in assemblies of parts. This is used to define part tolerances, and later verify that manufacturing processes are statistically capable of producing parts to this tolerance spec. Generally, the goal is to specify the widest possible tolerances to minimize scrap ($$$) while ensuring any combination of parts within these tolerances still fit together and function. GD&T (ASME Y14.5) is commonly used as the languge to express three-dimensional tolerances.

This application does not attempt to model all of the tolerances in your assembly, rather, this is a tool to help you model and understand critical tolerance stacks in one dimension. This greatly simplifies the modelling process and generally makes for much clearer, actionable, output. To construct a 1D model, you will need to:

1. Determine the target measurement you want to evaluate.
2. Define an axis to project this measurement onto (often times you can just project onto a plane).
3. Define the positive and negative directions along your axis - this is very important!
4. Determine the chain of dimensions needed to define the stackup that results in your target measurement.
5. Using this chain of dimensions, record the dimensions and tolerances as projected on your axis, making sure the signs are correct.

## Build Instructions

1. Install Rust via [Rustup](https://www.rust-lang.org/tools/install).
2. Clone the repository with `git clone https://github.com/aevyrie/tolstack.git`
3. From the `tolstack` directory, run `cargo run --release` to build and launch the application with compiler optimizations.

### Hardware Requirements and Software Dependencies

* Make sure your graphics drivers are up to date!
* Linux/Windows: You will need a modern graphics card that supports Vulkan
  * Integrated graphics (Intel HDxxx) requires vulkan support, check [here](https://www.intel.com/content/www/us/en/support/articles/000005524/graphics.html)
* MacOS: the backend uses Metal, check [here](https://en.wikipedia.org/wiki/Metal_(API)#Supported_GPUs) for requirements
