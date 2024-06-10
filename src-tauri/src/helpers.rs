use crate::arm7::Processor;
use regex::Regex;

/// Regex expression for every condition code.
pub fn condition_codes() -> &'static str {
    r"(eq|ne|cs|hs|cc|lo|mi|pl|vs|vc|hi|ls|ge|lt|gt|le|al)"
}

pub fn register() -> &'static str {
    r"(r\d+|sp|lr|pc)"
}

pub fn mnemonic_extension() -> &'static str {
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
    r"#-?(0b[01]+|0x[A-Fa-f\d]+|\d+)"
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
pub fn get_all_numbers(line: &str) -> Result<Vec<i64>, Vec<String>> {
    let mut errors: Vec<String> = Vec::new();
    let mut numbers: Vec<i64> = Vec::new();

    for mat in Regex::new(format!(r"(r\d+|lr|sp|pc|#[\da-fA-Fx]+|{})", i_number()).as_str())
        .unwrap()
        .find_iter(line)
        .map(|m| m.as_str())
    {
        if mat.starts_with('r') {
            // handle register numbers
            match (&mat[1..]).parse::<i64>() {
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
            // handle immediate values
            // check for negative value
            let mut sign: i64 = 1;
            let mut index_offset = 0; // if there is a minus sign, offset the index by one.
            if mat.starts_with("#-") {
                sign = -1;
                index_offset = 1;
            }
            let num = if is_bin(mat) {
                // Binary
                i64::from_str_radix(&mat[3 + index_offset..], 2)
            } else if is_hex(mat) {
                // Hexadecimal
                i64::from_str_radix(&mat[3 + index_offset..], 16)
            } else if is_dec(mat) {
                // Immediate Decimal Value
                (&mat[1 + index_offset..]).parse::<i64>()
            } else {
                // invalid immediate value
                errors.push(format!("{} is not a valid immediate value.", mat));
                continue;
            };
            // check for out of bounds error
            if let Ok(num) = num {
                // acceptable values are -2^31 to 2^32 - 1 inclusive
                if i64::from(i32::MIN) <= num && num <= i64::from(u32::MAX) {
                    numbers.push(sign * num);
                } else {
                    errors.push(format!("Immediate value {} is out of bounds.", mat));
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

pub fn set_nz_flags(num: u32, chip: &mut Processor) {
    // set aspr flags
    chip.N = (num as i32) < 0;
    chip.Z = num == 0;
}

pub fn is_Rd_immed(mnemonic: &str, line: &str) -> bool {
    Regex::new(
        format!(
            r"^{}{}\s+{}\s*,\s*{}$",
            mnemonic,
            mnemonic_extension(),
            register(),
            i_number()
        )
        .as_str(),
    )
    .unwrap()
    .is_match(line)
}

pub fn is_Rd_Rm(mnemonic: &str, line: &str) -> bool {
    Regex::new(
        format!(
            r"^{}{}\s+{}\s*,\s*{}$",
            mnemonic,
            mnemonic_extension(),
            register(),
            register()
        )
        .as_str(),
    )
    .unwrap()
    .is_match(line)
}

pub fn is_Rd_Rn_immed(mnemonic: &str, line: &str) -> bool {
    Regex::new(
        format!(
            r"^{}{}\s+{}\s*,\s*{}\s*,\s*{}$",
            mnemonic,
            mnemonic_extension(),
            register(),
            register(),
            i_number()
        )
        .as_str(),
    )
    .unwrap()
    .is_match(line)
}
pub fn is_Rd_Rn_Rm(mnemonic: &str, line: &str) -> bool {
    Regex::new(
        format!(
            r"^{}{}\s+{}\s*,\s*{}\s*,\s*{}$",
            mnemonic,
            mnemonic_extension(),
            register(),
            register(),
            register()
        )
        .as_str(),
    )
    .unwrap()
    .is_match(line)
}
