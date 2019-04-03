# `rsgen`
`rsgen` is a tiny library and command line tool that generates random characters string(s).

## Use as library

[![documentation in docs.rs](https://docs.rs/rsgen/badge.svg)](https://docs.rs/rsgen)
[![LICENSE](https://img.shields.io/github/license/sadaie/rsgen.svg?style=flat)](LICENSE)

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rsgen = "0.2"
```

### Examaple

```rust
use rsgen::{gen_random_string, OutputCharsType};

let output_chars_type = OutputCharsType::LatinAlphabetAndNumeric {
    use_upper_case: true,
    use_lower_case: true,
};
let _random_string = gen_random_string(32, output_chars_type);
```

## Install CLI tool

**`rsgen` is written in Rust. Thus you should install the latest Rust ecosystem in advance.**  
**refs. [rustup](https://rustup.rs/)**

### With `cargo install`

```
$ cargo install -f rsgen
```

### Build from source code

```
$ git clone https://github.com/sadaie/rsgen.git
$ cd rsgen
$ cargo build --release
$ ls target/release/
build       deps        examples    incremental native      rsgen      rsgen.d
```

## Usage

### Generating a random characters string.

```
$ rsgen
V05ZHhKa
```

### Generating a specified-length rondom characters string.

```
$ rsgen -c 12
TpzjXxem3U5x
```

### Generating a specified-length rondom characters string for ten times.

```
$ rsgen -c 12 -l 10
2S18UasnECKx
xCLlTp4tZmn3
f9jsbGnSWZtB
jFrPnstxHsr8
K9xZAK0R8KHH
85EXezRgeceo
QOtY5BFwEZBl
HBaFlCFN7t9W
blOM7ZsviUBw
XBDSOETSLzUR
```

#### Additional options

- `-f`, `--fast` option sets to use the fast but *NOT* secure RNG, [Xorshift](https://en.wikipedia.org/wiki/Xorshift).
- `-n`, `--numeric` option sets to restrict the output to be numeric.
- `-p`, `--printable-ascii` option sets to use [the printable ASCII](https://en.wikipedia.org/wiki/ASCII#Printable_characters) *without* `SPACE`.
- `-P`, `--printable-ascii-with-space` option sets to use [the printable ASCII](https://en.wikipedia.org/wiki/ASCII#Printable_characters) *with* `SPACE`.
- `--only-upper-case` option sets to use upper case letters only.
- `--only-lower-case` option sets to use lower case letters only.
- `--only-latin-alphabet` option sets to use the Latin alphabet only, *not* includes numeric characters.

## License

MIT license.  
