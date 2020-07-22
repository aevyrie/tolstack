# Program Structure

The GUI is written using [iced](https://github.com/hecrj/iced/).

* `main.rs` is the entry point of the program, and contians the GUI code.
* The `ui\components` folder contains the (reusable) components that make up the ui
* `ui\style.rs` contains ui stylsheet and hot reloading logic
* The `io` folder contains file save/open dialog logic, as well as the serializable state of the application
* The `analysis` folder holds all of the actual tolerance simulation logic
