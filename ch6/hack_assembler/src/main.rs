use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::code::Code;
use crate::parser::AsmParser;

mod code;
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

    let mut machine_code = String::new();

    while parser.has_more_lines()? {
        let instruction = parser.line().unwrap();
        if instruction.is_some() {
            match Code::assemble(&instruction.as_ref().unwrap()).unwrap() {
                None => continue,
                Some(assembled) => {
                    machine_code = String::from(format!("{}\n{:0>16b}", machine_code, assembled));
                }
            }
        }
    }

    println!("{}", machine_code);

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
