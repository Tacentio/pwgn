use clap::Parser;
use password_gen::CharSet;
/// Password generation on the command line.
#[derive(Parser)]
pub struct Args {
    /// Character set to use. Options include: xkcd, ascii, asciiextended, numbers, and
    /// alphanumeric.
    #[clap(default_value = "xkcd")]
    pub character_set: CharSet,
    /// Length of password
    #[clap(default_value = "8")]
    pub length: u32,
    /// Number of passwords to generate
    #[clap(short, default_value = "1")]
    pub number: u32,
    /// Benchmark
    #[clap(short)]
    pub benchmark: bool,
}
