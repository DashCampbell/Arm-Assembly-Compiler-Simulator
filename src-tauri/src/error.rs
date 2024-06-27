use std::collections::VecDeque;

/// Contains functions to help with error handling.
use crate::arm7::{ConditionCode, ITStatus, Operands};
use std::str::FromStr;

/// Stores and handles general compile time errors
pub struct CompileErr {
    errors: Vec<String>,
    line_number: usize,   // line number of current file
    current_file: String, // index to file map of current file
}
impl CompileErr {
    pub fn new() -> Self {
        CompileErr {
            errors: Vec::new(),
            line_number: 1,
            current_file: "main.s".into(),
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
        // line_number from enumerator, so starts at one.
        self.line_number = line_number + 1;
    }
    pub fn update_current_file(&mut self, file_name: String) {
        self.current_file = file_name;
    }
    /// Appends a compile time error message
    pub fn push_message(&mut self, message: &str) {
        self.errors.push(format!(
            "\"{}\" line {}: {}",
            self.current_file, self.line_number, message
        ));
    }
    /// Appends a list of compile time error messages.
    pub fn extend(&mut self, errors: Vec<String>) {
        self.errors.extend(errors.into_iter().map(|err| {
            format!(
                "\"{}\" line {}: {}",
                self.current_file, self.line_number, err
            )
        }))
    }
    pub fn message(error: String) -> Vec<String> {
        vec![error]
    }
    /// Handles an IT instruction
    /// Because an IT instruction affects future instructions, if there is an error in the IT statement
    /// we cannot verify the correctness of the instructions within an IT block, so we return the compile
    /// errors immediately.
    pub fn handle_it_instruction(
        mut self,
        it_block: &mut VecDeque<ConditionCode>,
        line: String,
    ) -> Result<Self, Vec<String>> {
        // check if IT block is within another IT block
        if !it_block.is_empty() {
            self.push_message("IT statement cannot be inside another IT block.");
            return Err(self.errors);
        }
        // split line into IT<x<y<z>>> and condition code.
        let line = line.split_whitespace().collect::<Vec<&str>>();

        // get the default condition statement
        let default_cc = if let Some(cc) = line.get(1) {
            match ConditionCode::from_str(cc) {
                Ok(cc) => cc,
                Err(err) => {
                    self.push_message(err.as_str());
                    return Err(self.errors);
                }
            }
        } else {
            self.push_message("IT statement must have a base condition.");
            return Err(self.errors);
        };
        // get the list of if else conditions
        for (index, c) in line[0][1..].chars().enumerate() {
            if index > 4 {
                self.push_message("An IT statement can only have conditions for 4 instructions.");
                return Err(self.errors);
            }
            if c == 't' {
                it_block.push_back(default_cc);
            } else {
                // c == 'e'
                it_block.push_back(default_cc.opposite_condition());
            }
        }
        Ok(self)
    }
    pub fn get_it_status(
        &mut self,
        it_block: &mut VecDeque<ConditionCode>,
        cc: Option<ConditionCode>,
    ) -> ITStatus {
        // if in IT block, validate condition code
        if let Some(correct_cc) = it_block.pop_front() {
            // check condition code for errors
            if let Some(cc) = cc {
                if cc != correct_cc {
                    self.push_message("The condition code must be the same or opposite of the IT block's condition code.");
                }
            } else {
                self.push_message("Instruction inside an IT block must have a condition code.");
            }
            if it_block.is_empty() {
                ITStatus::LAST
            } else {
                ITStatus::IN
            }
        } else {
            ITStatus::OUT
        }
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
