/// Contains functions to help with error handling.
use crate::arm7::Operands;

/// Stores and handles general compile time errors
pub struct CompileErr {
    errors: Vec<String>,
    line_number: usize,
}
impl CompileErr {
    pub fn new() -> Self {
        CompileErr {
            errors: Vec::new(),
            line_number: 1,
        }
    }
    /// Returns the operands if there are no compile errors, otherwise returns a list of compile errors.
    pub fn result(self) -> Result<(), Vec<String>> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }
    /// For returning the current list of compile errors early
    pub fn early_return(self) -> Result<(), Vec<String>> {
        Err(self.errors)
    }
    /// Updates the line number for error messages
    pub fn update_line_number(&mut self, line_number: usize) {
        // The line number passed is from an iterator, so the parameter starts at zero.
        self.line_number = line_number + 1;
    }
    /// Appends a compile time error message
    pub fn push_message(&mut self, message: &str) {
        self.errors
            .push(format!("Line {}: {}", self.line_number, message));
    }
    /// Appends a list of compile time error messages.
    pub fn extend(&mut self, errors: Vec<String>) {
        self.errors.extend(
            errors
                .into_iter()
                .map(|err| format!("Line {}: {}", self.line_number, err)),
        )
    }
    pub fn message(error: String) -> Vec<String> {
        vec![error]
    }
}

/// Contains compile time errors for instructions, and error handling functions related to instructions.
pub struct InstructionCompileErr(Vec<String>);
impl InstructionCompileErr {
    pub fn new() -> Self {
        InstructionCompileErr(Vec::new())
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
            self.0.push("Immediate value must be within 8 bits.".into());
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
    /// Pushes error message when S extension is not allowed
    pub fn invalid_s_extension(&mut self, s: bool) {
        if s {
            self.0
                .push("S extension is not allowed for this instruction.".to_string());
        }
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
