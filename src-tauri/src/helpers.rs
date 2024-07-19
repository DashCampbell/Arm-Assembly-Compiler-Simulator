use crate::arm7::Processor;
use regex::Regex;

/// Regex expression for every condition code.
pub fn condition_codes() -> &'static str {
    r"(eq|ne|cs|hs|cc|lo|mi|pl|vs|vc|hi|ls|ge|lt|gt|le|al)"
}

fn register() -> &'static str {
    r"\s*(r\d+|sp|lr|pc)\s*"
}

fn mnemonic_extension() -> &'static str {
    r"s?(eq|ne|cs|hs|cc|lo|mi|pl|vs|vc|hi|ls|ge|lt|gt|le|al)?(.w)?"
}

/// Regex expression for unsigned immediate values
/// ex: #0x12, #12, #0b1100
pub fn u_number() -> &'static str {
    r"#(0b[01]+|0x[A-Fa-f\d]+|\d+)"
}
/// Regex expression for signed immediate values
/// ex: #0x12, #-12, #-0b1100, #12
pub fn i_number() -> &'static str {
    r"\s*#-?(0b[01]+|0x[A-Fa-f\d]+|\d+)\s*"
}
pub fn is_bin(num: &str) -> bool {
    Regex::new(r"^#-?0b[01]+$").unwrap().is_match(num)
}
pub fn is_hex(num: &str) -> bool {
    Regex::new(r"^#-?0x[A-Fa-f\d]+$").unwrap().is_match(num)
}
pub fn is_dec(num: &str) -> bool {
    Regex::new(r"#-?\d+$").unwrap().is_match(num)
}

/// Collect all unsigned/signed numbers in a line. Including register numbers, hexadecimal, binary, immediate values, etc..
/// Returns an error if there are invalid registers, numbers are invalid, out of bounds, etc...
pub fn get_all_numbers(line: &str) -> Result<Vec<u32>, Vec<String>> {
    let mut errors: Vec<String> = Vec::new();
    let mut numbers: Vec<u32> = Vec::new();

    for mat in Regex::new(format!(r"{}|#[\da-fA-Fx]+|{}", register(), i_number()).as_str())
        .unwrap()
        .find_iter(line)
        .map(|m| m.as_str())
    {
        let mat = mat.trim();
        if mat.starts_with('r') {
            // handle register numbers
            match (&mat[1..]).parse::<u32>() {
                Ok(n) => {
                    if n > 15 {
                        errors.push(format!(
                            "Register r{} is invalid, only registers r0 to r15 are allowed.",
                            n
                        ));
                    } else {
                        numbers.push(n);
                    }
                }
                Err(_) => errors.push(format!(
                    "Register {} is invalid, only registers r0 to r15 are allowed.",
                    mat
                )),
            }
        // check special registers
        } else if mat == "sp" {
            numbers.push(13);
        } else if mat == "lr" {
            numbers.push(14);
        } else if mat == "pc" {
            numbers.push(15);
        } else {
            // Handle immediate values
            // check for negative value
            let (sign, index_offset) = if mat.starts_with("#-") {
                // if there is a minus sign, offset the index by one.
                (true, 1)
            } else {
                (false, 0)
            };
            // reject number if it cannot be contained in 32 bits.
            let num = if is_bin(mat) {
                // Binary
                u32::from_str_radix(&mat[3 + index_offset..], 2)
            } else if is_hex(mat) {
                // Hexadecimal
                u32::from_str_radix(&mat[3 + index_offset..], 16)
            } else if is_dec(mat) {
                // Immediate Decimal Value
                (&mat[1 + index_offset..]).parse::<u32>()
            } else {
                // invalid immediate value
                errors.push(format!("{} is not a valid immediate value.", mat));
                continue;
            };
            if let Ok(num) = num {
                // negate number if negative
                if sign {
                    numbers.push(num.wrapping_neg());
                } else {
                    numbers.push(num);
                }
            } else {
                errors.push(format!("Immediate value {} is out of bounds.", mat));
            }
        }
    }

    if errors.is_empty() {
        Ok(numbers)
    } else {
        Err(errors)
    }
}

/// Sets the N and Z aspr flags
pub fn set_nz_flags(num: u32, chip: &mut Processor) {
    // set aspr flags
    chip.N = (num as i32) < 0;
    chip.Z = num == 0;
}

#[allow(non_snake_case)]
pub fn is_Rd_immed(line: &str) -> bool {
    Regex::new(format!(r"^\S+\s+{},{}$", register(), i_number()).as_str())
        .unwrap()
        .is_match(line)
}

#[allow(non_snake_case)]
pub fn is_Rd_Rm(line: &str) -> bool {
    Regex::new(format!(r"^\S+\s+{},{}$", register(), register()).as_str())
        .unwrap()
        .is_match(line)
}
#[allow(non_snake_case)]
pub fn is_Rd_Rn_immed(line: &str) -> bool {
    Regex::new(format!(r"^\S+\s+{},{},{}$", register(), register(), i_number()).as_str())
        .unwrap()
        .is_match(line)
}
#[allow(non_snake_case)]
pub fn is_Rd_Rn_Rm(line: &str) -> bool {
    Regex::new(format!(r"^\S+\s+{},{},{}$", register(), register(), register()).as_str())
        .unwrap()
        .is_match(line)
}

#[allow(non_snake_case)]
pub fn is_Rt_Rn(line: &str) -> bool {
    Regex::new(format!(r"^\S+\s+{},\s*\[{}]$", register(), register(),).as_str())
        .unwrap()
        .is_match(line)
}
#[allow(non_snake_case)]
pub fn is_Rt_Rn_imm(line: &str) -> bool {
    Regex::new(
        format!(
            r"^\S+\s+{},\s*\[{},{}]$",
            register(),
            register(),
            i_number()
        )
        .as_str(),
    )
    .unwrap()
    .is_match(line)
}
#[allow(non_snake_case)]
pub fn is_Rt_Rn_imm_post(line: &str) -> bool {
    Regex::new(
        format!(
            r"^\S+{},\s*\[{}]\s*,{}$",
            register(),
            register(),
            i_number()
        )
        .as_str(),
    )
    .unwrap()
    .is_match(line)
}
#[allow(non_snake_case)]
pub fn is_Rt_Rn_imm_pre(line: &str) -> bool {
    Regex::new(
        format!(
            r"^\S+\s+{},\s*\[{},{}]!$",
            register(),
            register(),
            i_number()
        )
        .as_str(),
    )
    .unwrap()
    .is_match(line)
}

/// Determines if line is in the format "mnemonic <extensions> <label>"
pub fn is_label(line: &str) -> bool {
    Regex::new(r"^\S+\s+\w+$").unwrap().is_match(line)
}
