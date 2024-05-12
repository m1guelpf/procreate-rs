# procreate-rs

> A simple library for parsing Procreate files in Rust.

[<img alt="github" src="https://img.shields.io/badge/github-m1guelpf/procreate--rs-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/m1guelpf/procreate-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/procreate.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/procreate)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-procreate-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/procreate)

This crate provides some simple utilities from reading metadata from Procreate files, and extracting a thumbnail or timelapse video from them.

```bash
cargo add procreate
```

## Usage

```rust,no_run
let file = procreate::File::open("example.procreate").unwrap();

let metadata = file.metadata().unwrap();
let thumbnail = file.thumbnail().unwrap();
let timelapse_segments = file.timelapse_segments().unwrap();
```

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
