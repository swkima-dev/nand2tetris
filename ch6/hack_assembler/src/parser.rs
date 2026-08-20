use anyhow::{Result, anyhow};
use regex::Regex;
use std::io::BufRead;

pub struct AsmParser<T: BufRead> {
    reader: T,
    _current_line: String,
    c_regex: Regex,
}

impl<T: BufRead> AsmParser<T> {
    pub fn new(reader: T) -> Self {
        Self {
            reader,
            _current_line: String::new(),
            c_regex: Regex::new(r"((?<dest>\w+)=)?((?<comp>\w+))?(;(?<jump>\w+))?")
                .expect("This regex is valid."),
        }
    }
    pub fn has_more_lines(&mut self) -> Result<bool> {
        Ok(!self.reader.fill_buf()?.is_empty())
    }
    pub fn line(&mut self) -> Result<Instruction> {
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        let instruction = buffer.trim();

        if instruction.starts_with("//") {
            return Ok(Instruction::Comment);
        } else if instruction.starts_with("@") {
            return Ok(Instruction::A(
                instruction.strip_prefix("@").unwrap().to_string(),
            ));
        } else if instruction.starts_with("(") && instruction.ends_with(")") {
            return Ok(Instruction::L(
                instruction
                    .strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .to_string(),
            ));
        } else {
            match self.c_regex.captures(instruction) {
                Some(caps) => {
                    return Ok(Instruction::C {
                        dest: caps.name("dest").map(|m| m.as_str().to_string()),
                        comp: caps.name("comp").map(|m| m.as_str().to_string()),
                        jump: caps.name("jump").map(|m| m.as_str().to_string()),
                    });
                }
                None => return Err(anyhow!("C Instruction parse error.")),
            }
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    A(String),
    C {
        dest: Option<String>,
        comp: Option<String>,
        jump: Option<String>,
    },
    L(String),
    Comment,
}
