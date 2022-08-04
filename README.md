# Rustspace

## Setup

- Add workspace members to a new Cargo.toml file

```toml title=Cargo.toml
[workspace]
members = ["blog_api", "blog_web", "blog_shared"]
```

- Now, build each member with the following commands.

```sh
cargo new --vcs none <package-name>
```

```sh
cargo new --vcs none --lib <library-name>
```

- `--vcs none` avoids creating a git repository with cargo command.

- To build the packages & library from the root directory.

```sh
cargo build
```

## Build

- Build a package with the `-p` flag.

```sh
cargo build -p blog_api
```
