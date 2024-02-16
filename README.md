## Dependencies
all dependency can be found in the [crates.io](https://crates.io) website

### Specify a dependency
```toml
[dependencies]
NAME = "VERSION"

rand = "0.8.4"
```

### Import the dependency
```rust
use rand::Rng;
```

### Install the dependencies
```bash
cargo build
```

## Run the project
```bash
cargo run -q
```

## Run the tests
```bash
cargo test
```

## Build the project
```bash
cargo build --release
```

## Check the project
```bash
cargo check
```
