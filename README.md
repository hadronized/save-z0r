# Save z0r.de

## How to build

You need [Rust](https://www.rust-lang.org). It’s recommended to install Rust via
[`rustup`](https://rustup.rs).

Then, simply run the following commands:

```
git clone https://github.com/phaazon/save-z0r
cd save-z0r
cargo build --release
```

## How to use

This project contains a scrapper you can use to scrap loops from z0r.de. Just give it the the `--from` and
`--to` argument to scrap ranges of loops.

> If you don’t provide those arguments, `--from` defaults to `0` and `--to` defaults to `7911`.

Currently, the scrapper only downloads the loops and puts them in your download directory. You can use the `--dest-dir` 
switch to put them in another directory.

Consult `--help` for further information on how to use the scrapper.

You can run the scrapper with the simple `cargo run --release --` command (put your argument after a space
after `--`.). For instance:

```
cargo run --release -- --to 10
```

Will download the first 11 (from 0 to 10) loops.
