/// Contains functions to help with error handling.
use crate::arm7::Operands;

/// Contains compile time errors, and error handling functions.
pub struct CompileErr(Vec<String>);
impl CompileErr {
    pub fn new() -> Self {
        CompileErr(Vec::new())
    }
    /// Returns the operands if there are no compile errors, otherwise returns a list of compile errors.
    pub fn result(self, operands: Operands) -> Result<Operands, Vec<String>> {
        if self.0.is_empty() {
            Ok(operands)
        } else {
            Err(self.0)
        }
    }
    /// Error if immediate value cannot be contained in 12 bits.
    pub fn check_imm8(&mut self, immed: u32) {
        if immed > u32::from(u8::MAX) {
            self.0
                .push("Immediate value must be within 12 bits.".into());
        }
    }

    /// Error if immediate value cannot be contained in 12 bits.
    pub fn check_imm12(&mut self, immed: u32) {
        if immed > (2 as u32).pow(12) - 1 {
            self.0
                .push("Immediate value must be within 12 bits.".into());
        }
    }
    /// Pushes error message if Stack Pointer is detected.
    pub fn check_sp(&mut self, r: u8, reg: &str) {
        if r == 13 {
            self.0
                .push(format!("{} is not allowed to be stack pointer.", reg));
        }
    }
    /// Pushes error message if Program Counter is detected. reg is register name, can be Rd, Rm, etc..
    pub fn check_pc(&mut self, r: u8, reg: &str) {
        if r == 15 {
            self.0
                .push(format!("{} is not allowed to be program counter.", reg));
        }
    }
    /// Pushes error message if Program Counter or Stack Pointer is detected.
    pub fn check_sp_or_pc(&mut self, r: u8, reg: &str) {
        self.check_pc(r, reg);
        self.check_sp(r, reg);
    }
    /// Error message when S extension is not allowed
    pub fn invalid_s_extension(&mut self) {
        self.0
            .push("S extension is not allowed for this instruction.".to_string());
    }
}

/// Invalid arguments message
pub fn invalid_args(line: &str) -> Vec<String> {
    vec![format!("\"{}\" contains invalid arguments", line)]
}
/// Run Time Error Message for incorrect operand types, ideally it will never be called.
pub fn invalid_operands() -> String {
    "Wrong arguments given.".into()
}
