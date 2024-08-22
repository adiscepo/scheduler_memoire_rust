# Scheduler EDF in Rust for RP2040

This project is a Rust implementation of a real-time scheduler using the Earliest Deadline First algorithm.

This is a translation of the C version developed here: [Scheduler EDF in C](https://github.com/adiscepo/scheduler_memoire_c)

It was produced as part of my Master's Thesis.

## Launch the project

You can flash the code directly onto the chip using 
```shell
elf2uf2 -d target/thumbv6m-none-eabi/release/scheduler-edf-rs
``` 
after compiling the `cargo build --release` code.

To obtain the code execution logs, a probe can be used. 
```shell
probe-rs run --chip RP2040 target/thumbv6m-none-eabi/release/scheduler-edf-rs
```