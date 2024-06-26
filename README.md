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

# Tests
## Run the tests
```bash
cargo test
```

## Run test help command
```bash
cargo test --help
```

## Run the test with the -- --help flag
```bash
cargo test -- --help
```

## Helpful flags
```bash
cargo test -- --help
cargo test -- --show-output
cargo test -- --nocapture
cargo test -- --ignored
cargo test -- --test-threads=1
cargo test -- --ignored --test-threads=1
cargo test -- --test integration_test
```

## Build the project with dev and release profile
```bash
cargo build

cargo build --release
``` 

## Check the project
```bash
cargo check
```

## Create doc for the project
```bash
cargo doc
```

## Open the doc in the browser
```bash
cargo doc --open
```

## Format all files according to standard
```bash
cargo fmt --all
```
or
```bash
find . -name "*.rs" -exec rustfmt {} \; 
```
