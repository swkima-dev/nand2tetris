use crate::code::Code;
use crate::parser::AsmParser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn assembler(path: &str) -> Result<String> {
    let reader = open(path)?;

    let mut parser = AsmParser::new(reader);

    let mut machine_code = String::new();

    while parser.has_more_lines()? {
        let instruction = parser.line().unwrap();
        if instruction.is_some() {
            match Code::assemble(&instruction.as_ref().unwrap()).unwrap() {
                None => continue,
                Some(assembled) => {
                    machine_code = String::from(format!("{}{:0>16b}\n", machine_code, assembled));
                }
            }
        }
    }

    Ok(machine_code)
}
