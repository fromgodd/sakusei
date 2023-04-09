use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "saku", about = "A versatile alternative for 'touch' written in Rust that allows you to quickly create new files and boilerplates for popular programming languages.")]
pub struct Cli {
    #[structopt(help = "List of files or directories to create")]
    pub paths: Vec<PathBuf>,
    #[structopt(short, long, help = "Show this help message")]
    pub help: bool,
}

pub fn parse_args() -> Result<Vec<PathBuf>, errors::CustomError> {
    let args = Cli::from_args();
    if args.help {
        println!("{}", Cli::clap().get_help());
        std::process::exit(0);
    }
    if args.paths.is_empty() {
        return Err(errors::CustomError::NoArguments);
    }
    let mut paths = vec![];
    for path in args.paths {
        paths.push(path);
    }
    Ok(paths)
}
