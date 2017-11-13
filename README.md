# A better introduction to Rust FFI

This is a proof of concept code to show how to expose validation functionality from a Rust crate [iso8601](https://crates.io/crates/iso8601) to perl!

## Usage

This should work with Macs and Linux based environments. Also for perl FFI binding I'm using `FFI::Raw` module rather than xs.

- Install rust using rustup, accept the defaults and follow instructions: `curl https://sh.rustup.rs -sSf | sh`
- Clone the repo `git clone git@github.com:booyaa/iso8601_validator.git`
- Go to the `iso8601_validator` directory
- Build `cargo build`
- Install FFI::Raw `perl -MCPAN -e 'install FFI::Raw'
- Test code using perl script: `src/main.pl`

The Python code is @shepmaster's excellent [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/).

The awful perl code I claim as my own.

# Code of Conduct

Please see code-of-conduct.md which is based on [Contributor Covenant v1.4.1](https://www.contributor-covenant.org/)

# Contributions

Are always welcome provided you adhere to the Code of Conduct

# Copyright / License 

- Copyright Mark Sta Ana 2017.
- License is MIT (as per dependency).

Using module [iso8601](https://crates.io/crates/iso8601) copyright: 
- Hendrik Sollich
- Jan-Erik Rediger

