# Custom Math Lib

Custom function as alternatives for the awesome standard library that come with Rust.

## Directory

```ascii
.
|_ benches
|_ Cargo.lock
|_ Cargo.toml
|_ README.md
|_ src
|_ target
.
```

## Development

### Benchmarks

- Crates used:
  - Criterion
  - Cargo-Criterion

```sh
cargo bench -p "math_lib"
```

```sh
cargo criterion
```

### Testing

```sh
bacon test
```

```sh
cargo test --release
```

```sh
cargo watch -x 'test --release'
```
