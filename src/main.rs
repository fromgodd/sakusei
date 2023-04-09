use std::fs::File;
use std::io::prelude::*;
use clap::{Arg, App};
mod args;

fn main() -> std::io::Result<()> {
    let matches = App::new("sakusei")
                          .version("0.1.0")
                          .subcommand(args::new_file_subcommand())
                          .get_matches();

    match matches.subcommand() {
        Some(("new", new_matches)) => {
            let filename = new_matches.value_of("filename").unwrap();
            let mut file = File::create(filename)?;
            file.write_all(b"Hello, world!")?;
        },
        _ => {}
    }

    Ok(())
}
