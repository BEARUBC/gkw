# Rust-Systems
This is the main asynchronous, non-blocking event loop running the system processes on the Raspberry Pi 4.

## Project Directory
    root
        |-> py
            |-> (Python analytics software)
        |-> py_io
            |-> (intermediary buffers for communications/data-passing with the Python analytics software)
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
The asynchronous, non-blocking runtime is provided by [actix](https://crates.io/crates/actix). Serialization/Deserialization is provided by [serde_json](https://crates.io/crates/serde_json). The Pololu Micro Maestro interface library is provided by [raestro](https://crates.io/crates/raestro), a custom-built solution.
