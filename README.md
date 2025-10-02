# Rusty Shorty

URL shortener using SQLite and has a simple backoffice.

To run (you'll need Rustup and SQLite installed)

```sh
cargo run --package rusty-shorty --bin rusty-shorty
```

To build the final binary run

```sh
cargo build --release
```

The binary will be in `target/release/rusty-shorty` and will include the assets; embedded in the binary.

## Environment Variables

- `RUSTY_SHORTY_CONFIG_PATH` - Path to the config file.

## Config File Example

```toml
[default.poem_public]
address = "127.0.0.1"
port = 8000

[default.poem_backoffice]
address = "127.0.0.1"
port = 8001

[default.sqlite]
path = "./sqlite.db"
```

## Default Credentials

```
username: admin
password: banana
```

That can be changed in the backoffice.