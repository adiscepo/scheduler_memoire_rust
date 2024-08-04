# blink-pico-rs

> This project mostly follows this excellent [template](https://github.com/rp-rs/blink-pico-rs),
> make sure to check it out!

To see this in action:

* install Rust with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* install the target for the Cortex-M0 with `rustup target install thumbv6m-none-eabi`
* connect the Pico to the USB port and do `cargo run`

[Screenshot](img/probe-run.png)

For debugging a Pico using another Pico, please:

* connect the Pico's as shown in the wiring diagram in the Appendix A of "Getting Started With Pico"
  official guide
* flash the Piocoprobe firmware to the Pico that will be the debugger. There is the Picoprobe firmware
  implementing the old custom Picoprobe protocol, and the firmware that implements the standard CMSIS-DAP.
  The former requires building a custom OpenOCD, and the latter can work with the mainstream debugging tools.
  Here, CMSIS-DAP is assumed, [the CMSIS-DAP Picoprobe](https://github.com/raspberrypi/picoprobe/releases).

The [Pico Debug Probe](https://www.raspberrypi.com/documentation/microcontrollers/debug-probe.html) is a
RP2040-based debug probe for the Pico.

Rust-based tools for deploying the firmware:

```sh
cargo install elf2uf2-rs --locked
cargo install probe-run
cargo install probe-rs-cli
cargo install cargo-embed
cargo install cargo-flash
cargo install flip-link
```

These tools provide the most cohesive experience when debugging the code in this repo:

```sh
# Also need to install vscode extension `probe-rs-debugger` from https://github.com/probe-rs/vscode/releases
# code --install-extension probe-rs-debugger-xxxxx.vsix 
# More of the configuration: https://probe.rs/docs/tools/vscode/
cargo install probe-rs-debugger

# Cortex-M Debug extension
code --install-extension marus25.cortex-debug
```

If the preference is to use a standalone OpenOCD for debugging (e.g. you need to debug code on several cores
which `probe-rs-debugger` doesn't support at the time of writing), install `OpenOCD`, start it with

```sh
openocd -f interface/cmsis-dap.cfg -f target/rp2040.cfg -s tcl -c "adapter speed 5000"
```

and finally use the "External OpenOCD" profile for debugging.

To debug outside of the VS Code, you'll need `gdb`:

```sh
# Ubuntu
sudo apt install gdb-multiarch

# mac OS
brew install armmbed/formulae/arm-none-eabi-gcc
softwareupdate --install-rosetta
```

and then

```sh
# Ubuntu:
gdb-multiarch -q -ex "target extended-remote :3333" target/thumbv6m-none-eabi/debug/blink-pico-rs

# mac OS:
arm-none-eabi-gdb -q -ex "target extended-remote :3333" target/thumbv6m-none-eabi/debug/blink-pico-rs
```

It certainly benefits to look at the code in the official [C SDK repo](https://github.com/raspberrypi/pico-sdk)
repository and in the [Examples](https://github.com/raspberrypi/pico-sdk) one.

To learn much more:

* [Pico documentation](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html)
* [Getting started with Pico](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf)
* [Rust Embedded Bookshelf](https://docs.rust-embedded.org/)
* [Discovery book](https://docs.rust-embedded.org/discovery/index.html)
* [Rust Embedded Book](https://docs.rust-embedded.org/book/index.html)
* [`probe-rs` Documentation](https://probe.rs/)
