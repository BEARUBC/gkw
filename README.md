# Rust-Systems
This is the main asynchronous, non-blocking event loop running the system processes on the Raspberry Pi 4.

## Project Directory
    root
        |-> hooks (git hooks, sym-linked from .git/hooks/)
        |-> py (Python analytics software)
        |-> py_io (intermediary buffers for communications/data-passing with the Python analytics software)
        |-> src
            |-> actor
                |-> critical actor
                |-> non critical actor
                |-> ping
                |-> ping response
            |-> json_io
            |-> main
            |-> state machine

Respective READMEs *should* be in each folder.

## Dependencies
The asynchronous, non-blocking runtime is provided by [actix](https://crates.io/crates/actix). Serialization/Deserialization is provided by [serde_json](https://crates.io/crates/serde_json). The Pololu Micro Maestro interface library is provided by [raestro](https://crates.io/crates/raestro), a custom-built solution. Lower level GPIO functionalities are provided by [rppal](https://crates.io/crates/rppal). Keep in mind, that *rppal* can only be built on a Raspberry Pi. Builds on other machines *will* fail. The bindings to the Python Interpreter are provided by [pyo3](https://crates.io/crates/pyo3).

Information on dependencies can be found in their respective documentation pages.
