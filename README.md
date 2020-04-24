# mctol: Monte-Carlo Tolerance Simulation

Basic 1D Monte-Carlo simulation of tolerance chains. Currently supports:

Interfaces

* Linear dimensions (point A to B)
* Hole/pin pairs
* 2 hole/pin pairs in parallel (a part with 2 pins located in another part with 2 holes)

Tolerances

* Unilateral tolerances (+0/-0.5)
* Equal bilateral tolerances (+/-0.5)
* Unequal bilateral tolerances (5.2mm +0.2/-0.1)

Input/Output

* Saving/loading JSON files
* Exports simulation output data to CSV for plotting

Model data is currently input by defining in a JSON file.
