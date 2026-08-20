use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::parser::AsmParser;

mod parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let reader = open(&args.path)?;

    let mut parser = AsmParser::new(reader);

    while parser.has_more_lines()? {
        println!("{:?}", parser.line().unwrap())
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
