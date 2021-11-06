use structopt::StructOpt;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod hamming;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Pattern {
        pattern: String,
        
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    Hamming {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
}

fn main() {
    let args = Cli::from_args();

    match args.cmd {
        Command::Pattern { pattern, path } => {
            let file = File::open(&path)
                .expect("File could not be opened.");
            let reader = BufReader::new(file);

            for (_index, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                if line.contains(&pattern) {
                    println!("{}", line);
                }
            }
        },
        Command::Hamming { path } => {
            hamming::init(path);
        },
        _ => {
            println!("Unrecognized option.");
        }
    }
}