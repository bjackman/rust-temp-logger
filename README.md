I'm writing a temperature logger as a learning exercise for Rust.

The idea is to:

- Get readings from a digital temp sensor attached to a Raspberry Pi
- Stick em in an SQLite DB
- Generate a graph and serve it over HTTP

over-engineering as much as possible along the way.

Need:

- `sudo apt install gnuplot`
- `sudo apt install libsqlite3-dev`

# How to compile

For your local machine: `cargo build`.

For cross compilation, the official method [starts](https://rustup.rs/) by
piping a god ding dangus `curl` command into your shell.

Instead I found a better way [here](https://github.com/japaric/rust-cross#cross-compiling-with-cargo).

Here's how it looks to set up & build for ARMv7 on Ubuntu 18.04:

```
sudo apt install gcc-arm-linux-gnueabihf
