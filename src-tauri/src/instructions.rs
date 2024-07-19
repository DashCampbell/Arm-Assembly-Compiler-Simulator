use std::collections::HashMap;
use std::str::FromStr;

use crate::arm7::{MnemonicExtension, Operands, Processor};
use crate::error::InstructionCompileErr;
use crate::error::{self, check_memory_bounds};
use crate::helpers as hp;

/// Returns a Hashmap for all instructions, the key is the instruction's mnemonic
pub fn all_instructions() -> HashMap<String, Box<dyn Instruction>> {
    let mut instructions: HashMap<String, Box<dyn Instruction>> = HashMap::new();
    instructions.insert("mov".into(), Box::new(MOV {}));
    instructions.insert("add".into(), Box::new(ADD {}));
    instructions.insert("cmp".into(), Box::new(CMP {}));
    instructions.insert("b".into(), Box::new(B {}));
    instructions.insert("bl".into(), Box::new(BL {}));
    instructions.insert("strb".into(), Box::new(STRB {}));

    instructions
}
pub trait Instruction: Send + Sync {
    /// The instruction's mnemonic, must be lowercase for text parsing.
    fn mnemonic(&self) -> &'static str;
    /// Determines & validates the category and operands for an instruction line. Returns an error if the instruction is invalid.
    /// Called at compile time
    /// The extension if used to validate the instruction and get any constraints.
    fn get_operands(
        &self,
        extension: &MnemonicExtension,
        line: &str,
    ) -> Result<Operands, Vec<String>>;
    /// Returns Ok() if instruction executed correctly, returns Err() if there is a runtime error.
    /// Called at runtime.
    fn execute(
        &self,
        s_suffix: bool,
        operands: &Operands,
        chip: &mut Processor,
    ) -> Result<(), String>;
}

// Implement Instructions
pub struct MOV;
impl Instruction for MOV {
    fn mnemonic(&self) -> &'static str {
        "mov"
    }
    fn get_operands(
        &self,
        _extension: &MnemonicExtension,
        line: &str,
    ) -> Result<Operands, Vec<String>> {
        // get operands
        let mut errors = InstructionCompileErr::new();
        let operands = Operands::from_str(line)?;
        // check constraints
        match operands {
            Operands::Rd_immed { Rd, immed } => {
                errors.check_sp_or_pc(Rd, "Rd");
                errors.check_imm12(immed);
            }
            Operands::Rd_Rm { Rd, Rm, .. } => {
                errors.check_sp_or_pc(Rd, "Rd");
                errors.check_sp_or_pc(Rm, "Rm");
            }
            _ => return Err(error::invalid_args(line)),
        }
        errors.result(operands)
    }
    fn execute(
        &self,
        s_suffix: bool,
        operands: &Operands,
        chip: &mut Processor,
    ) -> Result<(), String> {
        // Get register and value to be moved into register.
        let (index, value) = match *operands {
            Operands::Rd_immed { Rd, immed } => (Rd as usize, immed),
            Operands::Rd_Rm { Rd, Rm, .. } => (Rd as usize, chip.R[Rm as usize]),
            _ => return Err(error::invalid_operands()),
        };
        // set aspr flags
        if s_suffix {
            hp::set_nz_flags(value, chip);
        }
        chip.R[index] = value;
        Ok(())
    }
}

pub struct ADD;
impl Instruction for ADD {
    fn mnemonic(&self) -> &'static str {
        "add"
    }
    fn get_operands(
        &self,
        _extension: &MnemonicExtension,
        line: &str,
    ) -> Result<Operands, Vec<String>> {
        let mut errors = InstructionCompileErr::new();
        let operands = Operands::from_str(line)?;
        // check constraints
        match operands {
            Operands::Rd_immed { immed, .. } => {
                errors.check_imm12(immed);
            }
            Operands::Rd_Rm { .. } => {}
            Operands::Rd_Rn_Rm { .. } => {}
            Operands::Rd_Rn_immed { immed, .. } => {
                errors.check_imm12(immed);
            }
            _ => return Err(error::invalid_args(line)),
        }
        errors.result(operands)
    }
    fn execute(
        &self,
        s_suffix: bool,
        operands: &Operands,
        chip: &mut Processor,
    ) -> Result<(), String> {
        let (index, a, b) = match *operands {
            Operands::Rd_immed { Rd, immed } => (usize::from(Rd), chip.R[Rd as usize], immed),
            Operands::Rd_Rm { Rd, Rm, shift: _ } => {
                (usize::from(Rd), chip.R[Rd as usize], chip.R[Rm as usize])
            }
            Operands::Rd_Rn_immed { Rd, Rn, immed } => {
                (usize::from(Rd), chip.R[Rn as usize], immed)
            }
            Operands::Rd_Rn_Rm {
                Rd,
                Rn,
                Rm,
                shift: _,
            } => (usize::from(Rd), chip.R[Rn as usize], chip.R[Rm as usize]),
            _ => return Err(error::invalid_operands()),
        };
        let (rd, carry) = a.overflowing_add(b);
        // set aspr flags
        if s_suffix {
            // set N and Z flags
            hp::set_nz_flags(rd, chip);
            // set Carry Flag
            chip.C = carry;
            // set Overflow Flag
            chip.V = (a as i32).overflowing_add(b as i32).1;
        }
        chip.R[index] = rd;
        Ok(())
    }
}

pub struct CMP;
impl Instruction for CMP {
    fn mnemonic(&self) -> &'static str {
        "CMP"
    }
    fn get_operands(
        &self,
        extension: &MnemonicExtension,
        line: &str,
    ) -> Result<Operands, Vec<String>> {
        let mut errors = InstructionCompileErr::new();
        let operands = Operands::from_str(line)?;

        // check constraints
        errors.invalid_s_extension(extension.s);
        match operands {
            Operands::Rd_immed { Rd, immed } => {
                if !extension.w {
                    errors.check_imm8(immed);
                }
                errors.check_pc(Rd, "Rd");
            }
            Operands::Rd_Rm { Rd, Rm, .. } => {
                errors.check_pc(Rd, "Rd");
                errors.check_sp_or_pc(Rm, "Rm");
            }
            _ => return Err(error::invalid_args(line)),
        }
        errors.result(operands)
    }
    fn execute(
        &self,
        _s_suffix: bool,
        operands: &Operands,
        chip: &mut Processor,
    ) -> Result<(), String> {
        let (a, b) = match *operands {
            Operands::Rd_immed { Rd, immed } => (chip.R[usize::from(Rd)], immed),
            Operands::Rd_Rm { Rd, Rm, shift: _ } => {
                (chip.R[usize::from(Rd)], chip.R[usize::from(Rm)])
            }
            _ => return Err(error::invalid_operands()),
        };
        // set aspr flags
        // set N and Z flags
        let (c, carry) = a.overflowing_sub(b);
        hp::set_nz_flags(c, chip);
        // set Carry Flag
        chip.C = !carry;
        // set Overflow Flag
        chip.V = (a as i32).overflowing_sub(b as i32).1;

        Ok(())
    }
}

pub struct B;

impl Instruction for B {
    fn mnemonic(&self) -> &'static str {
        "b"
    }
    /// this function never gets called
    fn get_operands(
        &self,
        _extension: &MnemonicExtension,
        _line: &str,
    ) -> Result<Operands, Vec<String>> {
        Ok(Operands::label { label: 0 })
    }
    fn execute(
        &self,
        _s_suffix: bool,
        operands: &Operands,
        chip: &mut Processor,
    ) -> Result<(), String> {
        match *operands {
            Operands::label { label } => {
                chip.R[15] = label as u32;
            }
            _ => return Err(error::invalid_operands()),
        }
        Ok(())
    }
}

pub struct BL;
impl Instruction for BL {
    fn mnemonic(&self) -> &'static str {
        "bl"
    }
    /// this function for this struct never gets called
    fn get_operands(
        &self,
        _extension: &MnemonicExtension,
        _line: &str,
    ) -> Result<Operands, Vec<String>> {
        Ok(Operands::label { label: 0 })
    }
    fn execute(
        &self,
        _s_suffix: bool,
        operands: &Operands,
        chip: &mut Processor,
    ) -> Result<(), String> {
        match *operands {
            Operands::label { label } => {
                chip.R[14] = chip.R[15]; // store PC register into Link register
                chip.R[15] = label as u32;
            }
            _ => return Err(error::invalid_operands()),
        }
        Ok(())
    }
}

pub struct STRB;

impl Instruction for STRB {
    fn mnemonic(&self) -> &'static str {
        "strb"
    }
    fn get_operands(
        &self,
        _extension: &MnemonicExtension,
        line: &str,
    ) -> Result<Operands, Vec<String>> {
        let errors = InstructionCompileErr::new();
        let operands = Operands::from_str(line)?;

        match operands {
            Operands::Rt_Rn_imm { .. }
            | Operands::Rt_Rn_imm_post { .. }
            | Operands::Rt_Rn_imm_pre { .. } => (),
            _ => return Err(error::invalid_args(line)),
        }
        errors.result(operands)
    }
    fn execute(
        &self,
        _s_suffix: bool,
        operands: &Operands,
        chip: &mut Processor,
    ) -> Result<(), String> {
        match *operands {
            Operands::Rt_Rn_imm { Rt, Rn, imm } => {
                // get byte
                let byte = chip.R[Rt as usize].to_le_bytes()[0];
                let address = error::check_memory_bounds(
                    chip.R[Rn as usize]
                        .overflowing_add_signed(imm.unwrap_or_default())
                        .0,
                    chip.memory.len(),
                )?;
                chip.memory[address] = byte;
            }
            Operands::Rt_Rn_imm_post { Rt, Rn, imm } => {
                let byte = chip.R[Rt as usize].to_le_bytes()[0];
                let address = error::check_memory_bounds(chip.R[Rn as usize], chip.memory.len())?;
                chip.memory[address] = byte;
                chip.R[Rn as usize] = chip.R[Rn as usize].overflowing_add_signed(imm).0;
            }
            Operands::Rt_Rn_imm_pre { Rt, Rn, imm } => {
                let byte = chip.R[Rt as usize].to_le_bytes()[0];
                chip.R[Rn as usize] = chip.R[Rn as usize].overflowing_add_signed(imm).0;
                let address = error::check_memory_bounds(chip.R[Rn as usize], chip.memory.len())?;
                chip.memory[address] = byte;
            }
            _ => return Err(error::invalid_operands()),
        }
        Ok(())
    }
}
