# Minimal rp2040 template

Minimal template for using rust with rp2040.

## Prerequisites

Install Rust https://www.rust-lang.org/

## How to use this template

First install [cargo-generate](https://crates.io/crates/cargo-generate)

    cargo install cargo-generate

Generate new project based on the template

    cargo generate tanelikaivola/rp2040_minimal_template


## Building and installing template

### Setup for building the firmware

If you have fresh installation of rust, you need to add architecture as a target

    rustup target add thumbv6m-none-eabi

Then install [elf2uf2-rs](https://crates.io/crates/elf2uf2-rs) for automatic deploy of the build firmware to pico
    
    cargo install elf2uf2-rs

### Build

Got to your project folder (the name you gave in `cargo generate`)

    cd [your project name]

Connect pico into computer in bootmode and run `run` to build and deploy of the firmware

    cargo run --release

