# Rust worksapce base

## Overview

```sh
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── target
│   ├── CACHEDIR.TAG
│   └── debug
├── zap
│   ├── Cargo.toml
│   └── src
├── zap-add
│   ├── Cargo.toml
│   └── src
└── zap-reduce
    ├── Cargo.toml
    └── src
```

## New Cargo.toml

```sh
[workspace]
members=[
    "model",
    "view",
    "controller"
]
```

## Create a new bin create and 2 lib creates

```bash
cargo new zap
cargo new zap-add --lib
cargo new zap-reduce --lib
```

## zap cargo.toml dependencies

```bash
[dependencies]
zap-add = {path = "../zap-add"}
zap-reduce = {path = "../zap-reduce"}

```

## zap main.rs

```bash
use zap_add::add;
use zap_reduce::reduce;

fn main() {
    println!("Hello, world!");

    let r1 = add(12, 5);
    println!("r1 = {}", r1);

    let r2 = reduce(12, 5);
    println!("r2 = {}", r2);
}

```

## Running

```bash
cargo run
```
