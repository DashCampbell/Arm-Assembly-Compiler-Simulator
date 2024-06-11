/// Contains functions to help with error handling.

/// Invalid arguments message
pub fn invalid_args(line: &str) -> Vec<String> {
    vec![format!("\"{}\" contains invalid arguments", line)]
}
/// Run Time Error Message for incorrect operand types, ideally it will never be called.
pub fn invalid_operands() -> String {
    "Wrong arguments given.".into()
}

/// Check if immediate value can be contained in 12 bits.
pub fn check_imm12(immed: u32, errors: &mut Vec<String>) {
    if immed > (2 as u32).pow(12) - 1 {
        errors.push("Immediate value must be within 12 bits.".into());
    }
}

/// Pushes error message if Stack Pointer is detected.
pub fn check_sp(r: u8, reg: &str, errors: &mut Vec<String>) {
    if r == 13 {
        errors.push(format!("{} is not allowed to be stack pointer.", reg));
    }
}
/// Pushes error message if Program Counter is detected.
pub fn check_pc(r: u8, reg: &str, errors: &mut Vec<String>) {
    if r == 15 {
        errors.push(format!("{} is not allowed to be program counter.", reg));
    }
}
/// Pushes error message if Program Counter or Stack Pointer is detected.
pub fn check_sp_or_pc(r: u8, reg: &str, errors: &mut Vec<String>) {
    check_pc(r, reg, errors);
    check_sp(r, reg, errors);
}
