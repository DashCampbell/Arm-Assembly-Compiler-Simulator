use crate::arm7::{Encoding, Operands, Processor};
use crate::helpers as hp;

use regex::Regex;

pub trait Instruction: Send + Sync {
    fn mnemonic(&self) -> &'static str;
    fn valid_encodings(&self) -> &'static str;
    /// Determines & validates the encoding type and operands for an instruction line. Returns an error if the instruction is invalid.
    /// Called at compile time
    fn get_encoding(&self, line: &str) -> Result<(Encoding, Operands), Vec<String>>;
    // Encodes an instruction
    fn encode(&self, encoding: Encoding, operands: &Operands) -> String;
    /// Returns Ok() if instruction executed correctly, returns Err() if there is a runtime error.
    /// Called at runtime.
    fn execute(
        &self,
        encoding: Encoding,
        operands: &Operands,
        chip: &mut Processor,
    ) -> Result<(), String>;
}

// Implement Instructions
#[derive(Clone)]
pub struct MOV;
impl Instruction for MOV {
    fn mnemonic(&self) -> &'static str {
        "mov"
    }
    fn valid_encodings(&self) -> &'static str {
        "mov rd, rn\n
        mov rd, #immed"
    }
    fn get_encoding(&self, line: &str) -> Result<(Encoding, Operands), Vec<String>> {
        let line = line.trim();
        let s_flag = hp::check_s_flag(self.mnemonic(), line);

        // Remove mnemonic and flags first
        if let Some((_, line)) = line.split_once(' ') {
            let re_imm = Regex::new(format!(r"^r\d+\s*,\s*{}$", hp::u_number()).as_str()).unwrap(); // move immediate
            let re_reg = Regex::new(r"^r\d+\s*,\s*r\d+$").unwrap(); // move register

            // Trim whitespace
            let line = line.trim();
            let mut operands = Operands::new();

            if re_imm.is_match(line) {
                let encoding = if !s_flag {
                    Encoding::ImmT1
                } else {
                    Encoding::ImmT2
                };
                match hp::get_all_numbers(line) {
                    Ok(args) => {
                        let rd = args[0];
                        let immed = args[1];

                        operands.Rd = rd as u8;
                        operands.immed = immed as u32;

                        Ok((encoding, operands))
                    }
                    Err(errors) => Err(errors),
                }
            } else if re_reg.is_match(line) {
                let encoding = if !s_flag {
                    Encoding::RegT1
                } else {
                    Encoding::RegT2
                };
                match hp::get_all_numbers(line) {
                    Ok(args) => {
                        let rd = args[0];
                        let rn = args[1];

                        operands.Rd = rd as u8;
                        operands.Rn = rn as u8;

                        Ok((encoding, operands))
                    }
                    Err(errors) => Err(errors),
                }
            } else {
                Err(vec![format!(
                    "Invalid Arguments for intruction '{}'",
                    self.mnemonic()
                )])
            }
        } else {
            Err(vec![format!(
                "Not enough arguments for instruction '{}'",
                self.mnemonic()
            )])
        }
    }
    fn encode(&self, encoding: Encoding, operands: &Operands) -> String {
        match encoding {
            Encoding::ImmT1 => format!("0010 0{:03b} {:08b}", operands.Rd, operands.immed),
            _ => "".into(),
        }
    }
    fn execute(
        &self,
        encoding: Encoding,
        operands: &Operands,
        chip: &mut Processor,
    ) -> Result<(), String> {
        match encoding {
            Encoding::ImmT1 | Encoding::ImmT2 => chip.R[operands.Rd as usize] = operands.immed,
            Encoding::RegT1 | Encoding::RegT2 => {
                chip.R[operands.Rd as usize] = chip.R[operands.Rn as usize]
            }
            _ => return Err("Encoding for MOV instruction is wrong.".into()),
        }
        Ok(())
    }
}
