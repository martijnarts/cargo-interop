use quicli::prelude::*;
use structopt::StructOpt;

mod config;
mod languages;

use self::config::parse_config;

#[derive(Debug, StructOpt)]
#[structopt(name = "cargo-interop", about = "Easy interop between Rust and other languages")]
enum InteropCli {
    /// Parse config
    Parse {
        filename: String,
    },
    /// Build the right setup for your requirements
    Setup,
    /// Build interop files
    Build
}

fn main() -> CliResult {
    human_panic::setup_panic!();

    let opt = InteropCli::from_args();

    match opt {
        InteropCli::Parse { filename } => println!("{:?}", parse_config(filename).unwrap()),
        InteropCli::Setup => todo!(),
        InteropCli::Build {  } => todo!(),
    }

    Ok(())
}
