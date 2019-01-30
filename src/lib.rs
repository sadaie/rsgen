//! `rsgen` is a tiny library to generate random characters string.
//! 
//! ## Usage
//! 
//! ```
//! use rsgen::{gen_random_string, OutputCharsType};
//! 
//! let output_chars_type = OutputCharsType::LatinAlphabetAndNumeric {
//!     use_upper_case: true,
//!     use_lower_case: true,
//! };
//! let _random_string = gen_random_string(32, output_chars_type);
//! ```
//! 

use rand::{self, Rng};

/// Configuration for output characters.
#[derive(Clone, Copy)]
pub enum OutputCharsType {
    /// Latin-Alphabet specifying to use upper/lower case.
    LatinAlphabet {
        use_upper_case: bool,
        use_lower_case: bool,
    },
    /// Latin-Alphabet and numeric figures specifying to use upper/lower case.
    LatinAlphabetAndNumeric {
        use_upper_case: bool,
        use_lower_case: bool,
    },
    /// Numeric figures.
    Numeric,
    /// Printable ASCII characters *without* SPACE. (0x21-0x7E)
    PrintableAsciiWithoutSpace,
    /// Printable ASCII characters *with* SPACE. (0x20-0x7E)
    PrintableAsciiWithSpace,
}

/// Generates a random characters string.
/// 
/// This function uses [ThreadRng](https://docs.rs/rand/0.6.5/rand/rngs/struct.ThreadRng.html) in [rand crate](https://docs.rs/rand) internally.
/// 
/// # Example
/// 
/// ```
/// use rsgen::{gen_random_string, OutputCharsType};
/// 
/// let output_chars_type = OutputCharsType::LatinAlphabetAndNumeric {
///     use_upper_case: true,
///     use_lower_case: true,
/// };
/// let _random_string = gen_random_string(32, output_chars_type);
/// ```
pub fn gen_random_string(number_of_characters: usize, output_char_type: OutputCharsType) -> String {
    let mut rng = rand::thread_rng();
    gen_random_string_with_rng(&mut rng, number_of_characters, output_char_type)
}

/// Generates a random characters string specifying RNG.
/// 
/// # Example
/// 
/// ```
/// use std::time::SystemTime;
/// use rand_core::SeedableRng;
/// use rand_xorshift::XorShiftRng;
/// use rsgen::{gen_random_string_with_rng, OutputCharsType};
/// 
/// let output_chars_type = OutputCharsType::LatinAlphabetAndNumeric {
///     use_upper_case: true,
///     use_lower_case: true,
/// };
/// let now = SystemTime::now();
/// let seed = now
///     .duration_since(SystemTime::UNIX_EPOCH)
///     .map(|d| d.as_secs())
///     .unwrap();
/// let mut rng = XorShiftRng::seed_from_u64(seed);
/// let _random_string = gen_random_string_with_rng(&mut rng, 32, output_chars_type);
/// ```
pub fn gen_random_string_with_rng<R>(
    rng: &mut R,
    number_of_characters: usize,
    output_chars_type: OutputCharsType,
) -> String
where
    R: Rng,
{
    match output_chars_type {
        OutputCharsType::LatinAlphabet {
            use_upper_case,
            use_lower_case,
        } => {
            let range = match (use_upper_case, use_lower_case) {
                (true, true) => 26 + 26,
                _ => 26,
            };
            let charset: &[u8] = match (use_upper_case, use_lower_case) {
                (true, true) => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
                (true, false) => b"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                (false, true) => b"abcdefghijklmnopqrstuvwxyz",
                _ => unreachable!(),
            };
            let uniformed = rand::distributions::Uniform::from(0..range);
            rng.sample_iter(&uniformed)
                .take(number_of_characters)
                .map(|n| charset[n as usize] as char)
                .collect()
        }
        OutputCharsType::LatinAlphabetAndNumeric {
            use_upper_case,
            use_lower_case,
        } => {
            let range = match (use_upper_case, use_lower_case) {
                (true, true) => 26 + 26 + 10,
                _ => 26 + 10,
            };
            let charset: &[u8] = match (use_upper_case, use_lower_case) {
                (true, true) => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
                (true, false) => b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
                (false, true) => b"abcdefghijklmnopqrstuvwxyz0123456789",
                _ => unreachable!(),
            };
            let uniformed = rand::distributions::Uniform::from(0..range);
            rng.sample_iter(&uniformed)
                .take(number_of_characters)
                .map(|n| charset[n] as char)
                .collect()
        }
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
                .filter_map(std::char::from_u32)
                .collect()
        }
        OutputCharsType::PrintableAsciiWithSpace => {
            let uniform = rand::distributions::Uniform::from(0x20..=0x7e);
            rng.sample_iter(&uniform)
                .take(number_of_characters)
                .filter_map(std::char::from_u32)
                .collect()
        }
    }
}
