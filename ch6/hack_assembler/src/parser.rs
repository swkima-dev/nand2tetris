use anyhow::{Result, anyhow};
use regex::Regex;
use std::io::{BufRead, Seek};

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
            c_regex: Regex::new(
                r"((?<dest>[ADM]{1,3})=)?(?<comp>[!\-]?[01ADM](?:[+\-&|][1ADM])?)(;(?<jump>\w+))?",
            )
            .expect("This regex is valid."),
        }
    }

    pub fn reader_init(&mut self, reader: T) -> Result<()> {
        self.reader = reader;
        Ok(())
    }

    pub fn has_more_lines(&mut self) -> Result<bool> {
        Ok(!self.reader.fill_buf()?.is_empty())
    }
    pub fn line(&mut self) -> Result<Option<Instruction>> {
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        let instruction = buffer.trim();

        if instruction.starts_with("//") {
            return Ok(Some(Instruction::Comment));
        } else if instruction.is_empty() {
            return Ok(None);
        } else if instruction.starts_with("@") {
            return Ok(Some(Instruction::A(
                instruction.strip_prefix("@").unwrap().to_string(),
            )));
        } else if instruction.starts_with("(") && instruction.ends_with(")") {
            return Ok(Some(Instruction::L(
                instruction
                    .strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .to_string(),
            )));
        } else {
            match self.c_regex.captures(instruction) {
                Some(caps) => {
                    return Ok(Some(Instruction::C {
                        dest: caps.name("dest").map(|m| m.as_str().to_string()),
                        comp: caps
                            .name("comp")
                            .map(|m| m.as_str().to_string())
                            .filter(|s| !s.is_empty())
                            .expect("comp is necessary"),
                        jump: caps.name("jump").map(|m| m.as_str().to_string()),
                    }));
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
        comp: String,
        jump: Option<String>,
    },
    L(String),
    Comment,
}
