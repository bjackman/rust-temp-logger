I'm writing a temperature logger as a learning exercise for Rust.

The idea is to:

- Get readings from a digital temp sensor attached to a Raspberry Pi
- Stick em in an SQLite DB
- Generate a graph and serve it over HTTP

over-engineering as much as possible along the way.

Need:

- `sudo apt install gnuplot`
- `sudo apt install libsqlite3-dev`
