//! # `rsgen`
//! `rsgen` is a command line tool that generates random characters string(s).
//!
//! ## Install
//!
//! **`rsgen` is written in Rust. Thus you should install the latest Rust ecosystem in advance.**  
//! **refs. [rustup](https://rustup.rs/)**
//!
//! ### With `cargo install`
//!
//! ```
//! $ cargo install -f rsgen
//! ```
//!
//! ### Build from source code
//!
//! ```
//! $ git clone https://github.com/sadaie/rsgen.git
//! $ cd rsgen
//! $ cargo build --release
//! $ ls target/release/
//! build       deps        examples    incremental native      rsgen      rsgen.d
//! ```
//!
//! ## Usage
//!
//! ### Generating a random characters string.
//!
//! ```
//! $ rsgen
//! V05ZHhKa
//! ```
//!
//! ### Generating a specified-length rondom characters string.
//!
//! ```
//! $ rsgen -c 12
//! TpzjXxem3U5x
//! ```
//!
//! ### Generating a specified-length rondom characters string for ten times.
//!
//! ```
//! $ rsgen -c 12 -l 10
//! 2S18UasnECKx
//! xCLlTp4tZmn3
//! f9jsbGnSWZtB
//! jFrPnstxHsr8
//! K9xZAK0R8KHH
//! 85EXezRgeceo
//! QOtY5BFwEZBl
//! HBaFlCFN7t9W
//! blOM7ZsviUBw
//! XBDSOETSLzUR
//! ```
//!
//! #### Additional options
//!
//! - `-f`, `--fast` option sets to use the fast but *NOT* secure RNG, [Xorshift](https://en.wikipedia.org/wiki/Xorshift).
//! - `-n`, `--numeric` option sets to restrict the output to be numeric.
//! - `-p`, `--printable-ascii` option sets to use [the printable ASCII](https://en.wikipedia.org/wiki/ASCII#Printable_characters) *without* `SPACE`.
//! - `-P`, `--printable-ascii-with-space` option sets to use [the printable ASCII](https://en.wikipedia.org/wiki/ASCII#Printable_characters) *with* `SPACE`.
//! - `--only-upper-case` option sets to use upper case letters only.
//! - `--only-lower-case` option sets to use lower case letters only.
//! - `--only-latin-alphabet` option sets to use the Latin alphabet only, *not* includes numeric characters.
//!
//! ## License
//!
//! MIT license.  

use atty;
use clap;
use rand_core::SeedableRng;
use rand_xorshift;
use rsgen::{gen_random_string_with_rng, OutputCharsType};
use std::time::SystemTime;

fn argument_validator(v: String) -> Result<(), String> {
    let error_message = "The argument value must be 1 or greater.".to_owned();
    let value = v.parse::<usize>().map_err(|_| error_message.clone())?;
    if value > 0 {
        Ok(())
    } else {
        Err(error_message)
    }
}

fn main() {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            clap::Arg::with_name("count")
                .help("The number of characters to output.")
                .short("c")
                .long("count")
                .takes_value(true)
                .default_value("8")
                .validator(argument_validator)
                .value_name("NUMBER_OF_CHARACTERS"),
        )
        .arg(
            clap::Arg::with_name("lines")
                .help("The number of lines to output.")
                .short("l")
                .long("lines")
                .takes_value(true)
                .default_value("1")
                .validator(argument_validator)
                .value_name("NUMBER_OF_LINES"),
        )
        .arg(
            clap::Arg::with_name("numeric")
                .help("Restricts the output to be numeric.")
                .short("n")
                .long("numeric")
                .conflicts_with_all(&["printable-ascii", "printable-ascii-s"]),
        )
        .arg(
            clap::Arg::with_name("printable-ascii")
                .help("Uses the printable ASCII characters without SPACE. (0x21-0x7E)")
                .short("p")
                .long("printable-ascii")
                .conflicts_with_all(&["numeric", "printable-ascii-s"]),
        )
        .arg(
            clap::Arg::with_name("printable-ascii-s")
                .help("Uses the printable ASCII characters WITH SPACE. (0x20-0x7E)")
                .short("P")
                .long("printable-ascii-with-space")
                .conflicts_with_all(&["numeric", "printable-ascii"]),
        )
        .arg(
            clap::Arg::with_name("fast")
                .help("Uses fast but NOT secure random number generating algorithm.")
                .short("f")
                .long("fast"),
        )
        .arg(
            clap::Arg::with_name("only-upper-case")
                .help("Uses upper case letters only.")
                .long("only-upper-case")
                .conflicts_with("only-lower-case"),
        )
        .arg(
            clap::Arg::with_name("only-lower-case")
                .help("Uses lower case letters only.")
                .long("only-lower-case")
                .conflicts_with("only-upper-case"),
        )
        .arg(
            clap::Arg::with_name("only-latin-alphabet")
                .help("Uses the Latin alphabet only, NOT includes numeric characters.")
                .long("only-latin-alphabet")
                .conflicts_with_all(&["numeric", "printable-ascii", "printable-ascii-s"]),
        )
        .get_matches();

    let number_of_characters: usize = matches
        .value_of("count")
        .and_then(|c| c.parse().ok())
        .unwrap_or(8);
    let number_of_lines: usize = matches
        .value_of("lines")
        .and_then(|l| l.parse().ok())
        .unwrap_or(1);
    let should_use_fast_rng = matches.is_present("fast");
    let output_chars_type = if matches.is_present("numeric") {
        OutputCharsType::Numeric
    } else if matches.is_present("printable-ascii") {
        OutputCharsType::PrintableAsciiWithoutSpace
    } else if matches.is_present("printable-ascii-s") {
        OutputCharsType::PrintableAsciiWithSpace
    } else {
        let is_upper_only = matches.is_present("only-upper-case");
        let is_lower_only = matches.is_present("only-lower-case");
        let is_latin_only = matches.is_present("only-latin-alphabet");
        match (is_upper_only, is_lower_only, is_latin_only) {
            (true, false, true) => OutputCharsType::LatinAlphabet {
                use_upper_case: true,
                use_lower_case: false,
            },
            (true, false, false) => OutputCharsType::LatinAlphabetAndNumeric {
                use_upper_case: true,
                use_lower_case: false,
            },
            (false, true, true) => OutputCharsType::LatinAlphabet {
                use_upper_case: false,
                use_lower_case: true,
            },
            (false, true, false) => OutputCharsType::LatinAlphabetAndNumeric {
                use_upper_case: false,
                use_lower_case: true,
            },
            (false, false, true) => OutputCharsType::LatinAlphabet {
                use_upper_case: true,
                use_lower_case: true,
            },
            (false, false, false) => OutputCharsType::LatinAlphabetAndNumeric {
                use_upper_case: true,
                use_lower_case: true,
            },
            _ => unreachable!(),
        }
    };

    let is_stdout = atty::is(atty::Stream::Stdout);
    let printing = |(i, s)| {
        if i == (number_of_lines - 1) && !is_stdout {
            print!("{}", s);
        } else {
            println!("{}", s);
        }
    };

    let iterator = std::iter::repeat(()).take(number_of_lines);

    if should_use_fast_rng {
        let now = SystemTime::now();
        let seed = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap();
        let mut rng = rand_xorshift::XorShiftRng::seed_from_u64(seed);
        iterator
            .map(|_| gen_random_string_with_rng(&mut rng, number_of_characters, output_chars_type))
            .enumerate()
            .for_each(printing);
    } else {
        let mut rng = rand::thread_rng();
        iterator
            .map(|_| gen_random_string_with_rng(&mut rng, number_of_characters, output_chars_type))
            .enumerate()
            .for_each(printing);
    }
}
