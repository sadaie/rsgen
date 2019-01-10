//!  # `rsgen`
//!  `rsgen` is a command line tool that generates random characters string(s).
//!  
//!  ## Install
//!  
//!  **`rsgen` is written in Rust. Thus you should install the latest Rust ecosystem in advance.**  
//!  **refs. [rustup](https://rustup.rs/)**
//!  
//!  ### With `cargo install`
//!  
//!  ```
//!  $ cargo install -f rsgen
//!  ```
//!  
//!  ### Build from source code
//!  
//!  ```
//!  $ git clone https://github.com/sadaie/rsgen.git
//!  $ cd rsgen
//!  $ cargo build --release
//!  $ ls target/release/
//!  build       deps        examples    incremental native      rsgen      rsgen.d
//!  ```
//!  
//!  ## Usage
//!  
//!  ### Generating a random characters string.
//!  
//!  ```
//!  $ rsgen
//!  V05ZHhKa
//!  ```
//!  
//!  ### Generating a specified-length rondom characters string.
//!  
//!  ```
//!  $ rsgen -c 12
//!  TpzjXxem3U5x
//!  ```
//!  
//!  ### Generating a specified-length rondom characters string for ten times.
//!  
//!  ```
//!  $ rsgen -c 12 -l 10
//!  2S18UasnECKx
//!  xCLlTp4tZmn3
//!  f9jsbGnSWZtB
//!  jFrPnstxHsr8
//!  K9xZAK0R8KHH
//!  85EXezRgeceo
//!  QOtY5BFwEZBl
//!  HBaFlCFN7t9W
//!  blOM7ZsviUBw
//!  XBDSOETSLzUR
//!  ```
//!  
//!  #### Additional options
//!  
//!  - `-f`, `--fast` option sets to use the fast but *NOT* secure RNG, [Xorshift](https://en.wikipedia.org/wiki/Xorshift).
//!  - `-n`, `--numeric` option sets to restrict the output to be numeric.
//!  - `-p`, `--printable-ascii` options sets to use [the printable ASCII](https://en.wikipedia.org/wiki/ASCII#Printable_characters) *without* `SPACE`.
//!  - `-P`, `--printable-ascii-with-space` options sets to use [the printable ASCII](https://en.wikipedia.org/wiki/ASCII#Printable_characters) *with* `SPACE`.
//!  
//!  ## License
//!  
//!  MIT lincense.  

use atty;
use clap;
use lazy_static;
use rand::{self, Rng};
use rand_core::SeedableRng;
use rand_xorshift;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

lazy_static::lazy_static! {
    static ref FAST: Arc<Mutex<rand_xorshift::XorShiftRng>> = {
        let now = SystemTime::now();
        let seed = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap();
        let rng = rand_xorshift::XorShiftRng::seed_from_u64(seed);
        Arc::new(Mutex::new(rng))
    };
}

#[derive(Clone, Copy)]
enum OutputCharsType {
    Alphanumeric,
    Numeric,
    PrintableAsciiWithoutSpace,
    PrintableAsciiWithSpace,
}

fn gen_random_string_with_rng<R>(
    rng: &mut R,
    number_of_characters: usize,
    output_chars_type: OutputCharsType,
) -> String
where
    R: Rng,
{
    match output_chars_type {
        OutputCharsType::Alphanumeric => rng
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(number_of_characters)
            .collect(),
        OutputCharsType::Numeric => {
            let uniform = rand::distributions::Uniform::from(0..=9);
            rng.sample_iter(&uniform)
                .take(number_of_characters)
                .filter_map(|n| std::char::from_digit(n as u32, 10))
                .collect()
        }
        OutputCharsType::PrintableAsciiWithoutSpace => {
            let uniform = rand::distributions::Uniform::from(0x21..=0x7e);
            rng.sample_iter(&uniform)
                .take(number_of_characters)
                .filter_map(|n| std::char::from_u32(n))
                .collect()
        }
        OutputCharsType::PrintableAsciiWithSpace => {
            let uniform = rand::distributions::Uniform::from(0x20..=0x7e);
            rng.sample_iter(&uniform)
                .take(number_of_characters)
                .filter_map(|n| std::char::from_u32(n))
                .collect()
        }
    }
}

fn gen_random_string(
    number_of_characters: usize,
    output_chars_type: OutputCharsType,
    should_use_fast_rng: bool,
) -> String {
    if should_use_fast_rng {
        let fast = FAST.clone();
        let mut rng = fast.lock().unwrap();
        gen_random_string_with_rng(&mut *rng, number_of_characters, output_chars_type)
    } else {
        let mut rng = rand::thread_rng();
        gen_random_string_with_rng(&mut rng, number_of_characters, output_chars_type)
    }
}

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
        OutputCharsType::Alphanumeric
    };

    let is_stdout = atty::is(atty::Stream::Stdout);
    std::iter::repeat(())
        .take(number_of_lines)
        .map(|_| gen_random_string(number_of_characters, output_chars_type, should_use_fast_rng))
        .enumerate()
        .for_each(|(i, s)| {
            if is_stdout {
                println!("{}", s);
            } else {
                if i == (number_of_lines - 1) {
                    print!("{}", s);
                } else {
                    println!("{}", s);
                }
            }
        });
}
