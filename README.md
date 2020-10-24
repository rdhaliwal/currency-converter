# Currency Converter

This is a simple command line tool to convert currencies, and a way for me to learn Rust.
It is powered by the [Exchange Rates API](https://exchangeratesapi.io/)

## Running the app

```bash
./ccon 50 usd EUR
ccon 100 aud usd
```

## Development
### Setup:

- Prerequisites: have cargo and rust installed.
- Then run `cargo build` or `make build`

### Developing the app

```bash
cargo run 123.45 aud usd
cargo run 499 usd aud
```

### Formatting the source code

- Run `cargo fmt` or `make fmt`

### Testing

Todo.

### Creating a release

- Build a release by running `make create-release VERSION=v1.0.0`
- This will generate a file called `ccon-v1.0.0.tar.gz`.
- Extract this with `tar -xzf ccon-v1.0.0.tar.gz`

