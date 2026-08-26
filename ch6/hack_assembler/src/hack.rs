use crate::code::Code;
use crate::parser::{AsmParser, Instruction};
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
    let mut code = Code::new();

    let mut line_counter = 0;
    while parser.has_more_lines()? {
        let instruction = parser.line().unwrap();
        if let Some(instruction) = instruction {
            match instruction {
                Instruction::A(_) => line_counter += 1,

                Instruction::C {
                    dest: _,
                    comp: _,
                    jump: _,
                } => line_counter += 1,

                Instruction::L(symbol) => {
                    if !code.contains(&symbol) {
                        code.add_entry(symbol, line_counter);
                    }
                }
                _ => continue,
            }
        }
    }

    let reader = open(path)?;
    parser.reader_init(reader)?;
    let mut machine_code = String::new();

    while parser.has_more_lines()? {
        let instruction = parser.line().unwrap();
        eprintln!("{:?}", &instruction);
        if let Some(instruction) = instruction {
            match code.assemble(&instruction)? {
                None => continue,
                Some(assembled) => {
                    machine_code = format!("{}{:0>16b}\n", machine_code, assembled);
                }
            }
        }
    }

    Ok(machine_code)
}
