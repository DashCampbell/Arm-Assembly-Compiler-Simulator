use crate::arm7::{Category, MnemonicExtension, Operands, Processor};
use crate::error;
use crate::helpers as hp;

use regex::Regex;

pub trait Instruction: Send + Sync {
    /// The instruction's mnemonic, must be lowercase for text parsing.
    fn mnemonic(&self) -> &'static str;
    /// Determines & validates the category and operands for an instruction line. Returns an error if the instruction is invalid.
    /// Called at compile time
    /// The extension if used to validate the instruction and get any constraints.
    fn get_category(
        &self,
        extension: &MnemonicExtension,
        line: &str,
    ) -> Result<(Category, Operands), Vec<String>>;
    /// Returns Ok() if instruction executed correctly, returns Err() if there is a runtime error.
    /// Called at runtime.
    fn execute(
        &self,
        s_suffix: bool,
        category: Category,
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
    fn get_category(
        &self,
        _extension: &MnemonicExtension,
        line: &str,
    ) -> Result<(Category, Operands), Vec<String>> {
        // get operands
        let args = hp::get_all_numbers(line)?;
        let mut errors: Vec<String> = Vec::new();

        let mut operands = Operands::new();
        let category = if hp::is_Rd_immed(self.mnemonic(), line) {
            operands.immed = args[1] as u32;
            error::check_imm12(operands.immed, &mut errors);
            Category::Immediate
        } else if hp::is_Rd_Rm(self.mnemonic(), line) {
            operands.Rm = args[1] as u8;
            error::check_sp_or_pc(operands.Rm, "Rn", &mut errors);
            Category::Register
        } else {
            return Err(error::invalid_args(line));
        };
        operands.Rd = args[0] as u8;
        error::check_sp_or_pc(operands.Rd, "Rd", &mut errors);

        if errors.is_empty() {
            Ok((category, operands))
        } else {
            Err(errors)
        }
    }
    fn execute(
        &self,
        s_suffix: bool,
        category: Category,
        operands: &Operands,
        chip: &mut Processor,
    ) -> Result<(), String> {
        let rd = match category {
            Category::Immediate => operands.immed,
            Category::Register => chip.R[operands.Rm as usize],
            _ => return Err("Wrong encoding type given.".into()),
        };
        // set aspr flags
        if s_suffix {
            hp::set_nz_flags(rd, chip);
        }
        chip.R[operands.Rd as usize] = rd;

        Ok(())
    }
}

#[derive(Clone)]
pub struct ADD;

impl Instruction for ADD {
    fn mnemonic(&self) -> &'static str {
        "add"
    }
    fn get_category(
        &self,
        _extension: &MnemonicExtension,
        line: &str,
    ) -> Result<(Category, Operands), Vec<String>> {
        // get operands
        let args = hp::get_all_numbers(line)?;
        let mut errors: Vec<String> = Vec::new();

        let mut operands = Operands::new();
        let category = if hp::is_Rd_Rn_immed(self.mnemonic(), line) {
            operands.immed = args[2] as u32;
            error::check_imm12(operands.immed, &mut errors);
            Category::Immediate
        } else if hp::is_Rd_Rn_Rm(self.mnemonic(), line) {
            operands.Rm = args[2] as u8;
            Category::Register
        } else if hp::is_Rd_Rm(self.mnemonic(), line) {
            Category::Default
        } else {
            return Err(error::invalid_args(line));
        };
        operands.Rd = args[0] as u8;
        operands.Rn = args[1] as u8;

        if errors.is_empty() {
            Ok((category, operands))
        } else {
            Err(errors)
        }
    }
    fn execute(
        &self,
        s_suffix: bool,
        category: Category,
        operands: &Operands,
        chip: &mut Processor,
    ) -> Result<(), String> {
        let rn = chip.R[operands.Rn as usize];
        let num = match category {
            Category::Immediate => operands.immed,
            Category::Register => chip.R[operands.Rm as usize],
            Category::Default => chip.R[operands.Rd as usize],
        };
        let (rd, overflow) = rn.overflowing_add(num);
        // set aspr flags
        if s_suffix {
            // set N and Z flags
            hp::set_nz_flags(rd, chip);
            // TODO: Fix carry and overflow handling.
            // set Carry Flag
            chip.C = overflow;
            // set Overflow Flag
            chip.V = overflow && !(rn.checked_neg().is_some() != num.checked_neg().is_some());
        }
        chip.R[operands.Rd as usize] = rd;
        Ok(())
    }
}
