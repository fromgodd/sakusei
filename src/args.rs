use clap::{App, Arg, SubCommand};

pub fn new_file_subcommand() -> App<'static> {
    SubCommand::with_name("new")
        .about("Create a new file")
        .arg(Arg::with_name("filename").required(true).index(1))
}
