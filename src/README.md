# Program Structure - NOTE: Out of date since refactor

* `main.rs` is the entry point of the program, and contians the GUI code.
* `model.rs` contains all the simulation model logic.
* `tolerances.rs` holds definitions of the tolerance structures.

The GUI is written using [iced](https://github.com/hecrj/iced/).

## GUI Outline

Source

```
TolStack Application
├── page `Header`
|   └── component `EditableLabel`
└── page `Home`
    ├── container `InputPane`
    |   ├── component `NewToleranceEntry`
    |   └── component `ListToleranceEntries`
    └── container `OutputPane`
        ├── component `SimulationInput`
        └── component `SimulationOutput`
```

Files
```
└── src
    ├── main.rs
    ├── tolstack_analysis
    │   ├── analysis
    │   └── structures
    └── tolstack_ui
        ├── components
        │   ├── editable_label
        │   ├── new_tolerance_entry
        │   ├── list_tolerance_entries
        │   ├── simulation_input
        │   └── simulation_output
        ├── home.rs
        ├── header.rs
        └── style.rs
```
