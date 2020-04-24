# mctol: Monte-Carlo Tolerance Simulation

Basic 1D Monte-Carlo simulation of tolerance chains.

## Features

Mechanical Interfaces

* `Linear`: Linear dimensions (point A to B)
* `Float`: Single hole/pin pair (a part with one pin located in another part with one pin)
* `Compound`: Double hole/pin pairs in parallel (a part with two pins located in another part with two holes)

Tolerances

* Unilateral tolerances (+0/-0.5)
* Equal bilateral tolerances (+/-0.5)
* Unequal bilateral tolerances (5.2mm +0.2/-0.1)

Input/Output

* Saving/loading JSON files
* Exports simulation output data to CSV for plotting

## Input Example
The tolerance model is defined in a JSON file.

```JSON
{
  "params": {
    "part_sigma": 3.0,
    "assy_sigma": 4.0,
    "n_iterations": 10000000
  },
  "tolerance_loop": [
    {
      "Linear": {
        "distance": {
          "dim": 12.5,
          "tol_pos": 0.1,
          "tol_neg": 0.2,
          "sigma": 3.0
        }
      }
    }
  ]
}
```
