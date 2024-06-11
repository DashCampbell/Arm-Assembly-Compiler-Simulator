use crate::arm7::{MnemonicExtension, Operands, Processor};
use crate::error;
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
#[derive(Clone)]
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
        let mut errors: Vec<String> = Vec::new();
        let operands = Operands::from_str(line)?;
        // check constraints
        match operands {
            Operands::Rd_immed { Rd, immed } => {
                error::check_sp_or_pc(Rd, "Rd", &mut errors);
                error::check_imm12(immed, &mut errors);
            }
            Operands::Rd_Rm { Rd, Rm, .. } => {
                error::check_sp_or_pc(Rd, "Rd", &mut errors);
                error::check_sp_or_pc(Rm, "Rm", &mut errors);
            }
            _ => return Err(error::invalid_args(line)),
        }
        if errors.is_empty() {
            Ok(operands)
        } else {
            Err(errors)
        }
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

#[derive(Clone)]
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
        let mut errors: Vec<String> = Vec::new();
        let operands = Operands::from_str(line)?;
        // check constraints
        match operands {
            Operands::Rd_immed { immed, .. } => {
                error::check_imm12(immed, &mut errors);
            }
            Operands::Rd_Rm { .. } => {}
            Operands::Rd_Rn_Rm { .. } => {}
            Operands::Rd_Rn_immed { immed, .. } => {
                error::check_imm12(immed, &mut errors);
            }
            _ => return Err(error::invalid_args(line)),
        }
        if errors.is_empty() {
            Ok(operands)
        } else {
            Err(errors)
        }
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
