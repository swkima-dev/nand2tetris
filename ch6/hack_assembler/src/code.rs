use std::collections::HashMap;

use crate::parser::Instruction;
use anyhow::{Result, anyhow};

pub struct Code {
    symbols: HashMap<String, u16>,
    min_blank_address: u16,
}

impl Code {
    pub fn new() -> Self {
        let mut symbols: HashMap<String, u16> = HashMap::new();

        Self::symbols_init(&mut symbols);

        Self {
            symbols,
            min_blank_address: 16,
        }
    }

    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.symbols.insert(symbol, address);
    }

    fn add_variable(&mut self, symbol: String) -> u16 {
        let value_num = self.min_blank_address;
        self.add_entry(symbol, value_num);
        self.min_blank_address += 1;

        value_num
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.symbols.contains_key(symbol)
    }

    fn get_address(&self, symbol: &str) -> Option<&u16> {
        self.symbols.get(symbol)
    }

    pub fn assemble(&mut self, instruction: &Instruction) -> Result<Option<u16>> {
        match instruction {
            Instruction::A(symbol) => {
                if let Ok(number) = symbol.parse() {
                    if number > u16::MAX / 2 {
                        return Err(anyhow!("symbol number exceeded."));
                    }
                    return Ok(Some(number));
                }
                if let Some(address) = self.get_address(&symbol) {
                    return Ok(Some(*address));
                }
                if !self.contains(&symbol) {
                    return Ok(Some(self.add_variable(symbol.to_string())));
                }
                Err(anyhow!("invalid symbol."))
            }
            Instruction::C { dest, comp, jump } => {
                let dest_bits = Self::dest_assemble(dest);
                let (comp_bits, a_bool) = Self::comp_assemble(comp);
                let a_bit = a_bool as u16;
                let jump_bits = Self::jump_assemble(jump);
                Ok(Some(
                    (0b111 << 13) + (a_bit << 12) + (comp_bits << 6) + (dest_bits << 3) + jump_bits,
                ))
            }
            _ => Ok(None),
        }
    }

    fn symbols_init(symbols: &mut HashMap<String, u16>) {
        symbols.insert("R0".to_string(), 0);
        symbols.insert("R1".to_string(), 1);
        symbols.insert("R2".to_string(), 2);
        symbols.insert("R3".to_string(), 3);
        symbols.insert("R4".to_string(), 4);
        symbols.insert("R5".to_string(), 5);
        symbols.insert("R6".to_string(), 6);
        symbols.insert("R7".to_string(), 7);
        symbols.insert("R8".to_string(), 8);
        symbols.insert("R9".to_string(), 9);
        symbols.insert("R10".to_string(), 10);
        symbols.insert("R11".to_string(), 11);
        symbols.insert("R12".to_string(), 12);
        symbols.insert("R13".to_string(), 13);
        symbols.insert("R14".to_string(), 14);
        symbols.insert("R15".to_string(), 15);

        symbols.insert("SP".to_string(), 0);
        symbols.insert("LCL".to_string(), 1);
        symbols.insert("ARG".to_string(), 2);
        symbols.insert("THIS".to_string(), 3);
        symbols.insert("THAT".to_string(), 4);

        symbols.insert("SCREEN".to_string(), 16384);
        symbols.insert("KBD".to_string(), 24576);
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
            "M+1" => (0b110111, true),
            "M-1" => (0b110010, true),
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
                "JNE" => 0b101,
                "JLE" => 0b110,
                "JMP" => 0b111,
                &_ => unreachable!(),
            },
        }
    }
}
