use clap::Parser;
use password_gen::{self, PasswordGenerator, PasswordOptions};
use pwgn::config::Args;

fn main() {
    let cli_args = Args::parse();
    let mut generator = PasswordGenerator::new();
    let options = PasswordOptions::new(cli_args.length, cli_args.character_set);
    if cli_args.benchmark {
        for _i in 0..cli_args.number {
            generator.generate_password(&options);
        }
    } else {
        for _i in 0..cli_args.number {
            println!("{}", generator.generate_password(&options));
        }
    }
}
