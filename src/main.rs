mod task1;

use std::{fs, io::{BufReader, BufRead}};
use fs::File;
use clap::Parser;


static AUTHOR: &str = "Dominic Lindsay <babbleshack@babblebase.net>";
static VERSION: &str = "0.1";

#[derive(Parser, Debug)]
#[clap(version = VERSION, author = AUTHOR)]
struct InputArgs {
    // Path to puzzle input
    path: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
#[clap(version = VERSION, author = AUTHOR)]
    Task1A(Task1A),
    Task1B(Task1B),
    
}

#[derive(Parser, Debug)]
struct Task1A {}

#[derive(Parser, Debug)]
struct Task1B {}

fn load_data(path: &str) -> Vec<usize> {
    let file = File::open(path).expect("error opening input file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect()
}

fn main() {
    let opts: InputArgs = InputArgs::parse();
    let contents = load_data(opts.path.as_str());
    let depth: i32;
    match opts.subcmd {
        SubCommand::Task1A(_) => depth = task1::count_total_depth(&contents[..], 2),
        SubCommand::Task1B(_) => depth = task1::sum_windows(&contents[..], 3),
    }
    println!("Total increases: {}", depth);
}
