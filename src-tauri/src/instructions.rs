use crate::arm7::{MnemonicExtension, Operands, Processor};
use crate::error;
use crate::error::CompileErr;
use crate::helpers as hp;

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
        let mut errors = CompileErr::new();
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
        let mut errors = CompileErr::new();
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
        let mut errors = CompileErr::new();
        let operands = Operands::from_str(line)?;

        // check constraints
        if extension.s {
            errors.invalid_s_extension();
        }
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
