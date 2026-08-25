use crate::parser::Instruction;
use anyhow::{Result, anyhow};

pub struct Code {}

impl Code {
    pub fn assemble(instruction: &Instruction) -> Result<Option<u16>> {
        match instruction {
            Instruction::A(symbol) => {
                let number: u16 = symbol.parse()?;
                if number > u16::MAX / 2 - 1 {
                    return Err(anyhow!("symbol number exceeded."));
                }
                Ok(Some(number))
            }
            Instruction::L(_) => Err(anyhow!("unreachable!")),
            Instruction::C { dest, comp, jump } => {
                let dest_bits = Self::dest_assemble(dest);
                let (comp_bits, a_bool) = Self::comp_assemble(comp);
                let a_bit = a_bool as u16;
                let jump_bits = Self::jump_assemble(jump);
                Ok(Some(
                    (0b111 << 13) + (a_bit << 12) + (comp_bits << 6) + (dest_bits << 3) + jump_bits,
                ))
            }
            Instruction::Comment => Ok(None),
        }
    }

    fn dest_assemble(dest: &Option<String>) -> u16 {
        match dest {
            None => 0b000,
            Some(content) => match content.as_str() {
                "M" => 0b001,
                "D" => 0b010,
                "DM" => 0b011,
                "A" => 0b100,
                "AM" => 0b101,
                "AD" => 0b110,
                "ADM" => 0b111,
                _ => unreachable!(),
            },
        }
    }

    // return u16 is instruction[6..11], bool is instruction
    fn comp_assemble(comp: &str) -> (u16, bool) {
        match comp {
            "0" => (0b101010, false),
            "1" => (0b111111, false),
            "-1" => (0b111010, false),
            "D" => (0b001100, false),
            "A" => (0b110000, false),
            "!A" => (0b001101, false),
            "!D" => (0b110001, false),
            "-D" => (0b001111, false),
            "-A" => (0b110011, false),
            "D+1" => (0b011111, false),
            "A+1" => (0b110111, false),
            "D-1" => (0b001110, false),
            "A-1" => (0b110010, false),
            "D+A" => (0b000010, false),
            "D-A" => (0b010011, false),
            "A-D" => (0b000111, false),
            "D&A" => (0b000000, false),
            "D|A" => (0b010101, false),
            "M" => (0b110000, true),
            "!M" => (0b110001, true),
            "-M" => (0b110011, true),
            "M+1" => (0b110010, true),
            "D+M" => (0b000010, true),
            "D-M" => (0b010011, true),
            "M-D" => (0b000111, true),
            "D&M" => (0b000000, true),
            "D|M" => (0b010101, true),
            _ => unreachable!(),
        }
    }

    fn jump_assemble(jump: &Option<String>) -> u16 {
        match jump {
            None => 0b000,
            Some(content) => match content.as_str() {
                "JGT" => 0b001,
                "JEQ" => 0b010,
                "JGE" => 0b011,
                "JLT" => 0b100,
                "JME" => 0b101,
                "JLE" => 0b110,
                "JMP" => 0b111,
                &_ => unreachable!(),
            },
        }
    }
}
