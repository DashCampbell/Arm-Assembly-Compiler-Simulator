use crate::{
    arm7::{InputStatus, Label, MemSize, Operands, Processor},
    error,
};
use regex::Regex;

/// Regex expression for every condition code.
pub fn condition_codes() -> &'static str {
    r"(eq|ne|cs|hs|cc|lo|mi|pl|vs|vc|hi|ls|ge|lt|gt|le|al)"
}

fn register() -> &'static str {
    r"\s*(r\d+|sp|lr|pc)\s*"
}

#[warn(dead_code)]
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
pub fn re_label() -> &'static str {
    r"\s*[a-zA-Z_]\w*\s*"
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
#[allow(non_snake_case)]
pub fn is_Rt_Rn_Rm(line: &str) -> bool {
    Regex::new(
        format!(
            r"^\S+\s+{},\s*\[{},{}]$",
            register(),
            register(),
            register()
        )
        .as_str(),
    )
    .unwrap()
    .is_match(line)
}
#[allow(non_snake_case)]
pub fn is_Rt_Rn_Rm_shift(line: &str) -> bool {
    Regex::new(
        format!(
            r"^\S+\s+{},\s*\[{},{},\s*lsl\s*{}]$",
            register(),
            register(),
            register(),
            i_number(),
        )
        .as_str(),
    )
    .unwrap()
    .is_match(line)
}
#[allow(non_snake_case)]
/// ldr rt ,= <label>
pub fn is_Rt_equal_label(line: &str) -> bool {
    Regex::new(format!(r"^\S+{},={}$", register(), re_label()).as_str())
        .unwrap()
        .is_match(line)
}
#[allow(non_snake_case)]
/// ldr rt ,= imm32
pub fn is_Rt_equal_immed(line: &str) -> bool {
    Regex::new(format!(r"^\S+{},={}*$", register(), i_number()).as_str())
        .unwrap()
        .is_match(line)
}

/// Determines if line is in the format "mnemonic<extensions> <label>"
pub fn is_label(line: &str) -> bool {
    Regex::new(format!(r"^\S+{}$", re_label()).as_str())
        .unwrap()
        .is_match(line)
}

fn get_rt_and_address(operands: &Operands, chip: &mut Processor) -> Result<(u8, u32), String> {
    match *operands {
        Operands::Rt_Rn_imm { Rt, Rn, imm } => Ok((
            Rt,
            chip.R[Rn as usize]
                .overflowing_add_signed(imm.unwrap_or_default())
                .0,
        )),
        Operands::Rt_Rn_imm_post { Rt, Rn, imm } => {
            let address = chip.R[Rn as usize];
            chip.R[Rn as usize] = chip.R[Rn as usize].overflowing_add_signed(imm).0;
            Ok((Rt, address))
        }
        Operands::Rt_Rn_imm_pre { Rt, Rn, imm } => {
            chip.R[Rn as usize] = chip.R[Rn as usize].overflowing_add_signed(imm).0;
            Ok((Rt, chip.R[Rn as usize]))
        }
        Operands::Rt_Rn_Rm { Rt, Rn, Rm, shift } => Ok((
            Rt,
            chip.R[Rn as usize]
                .overflowing_add(
                    chip.R[Rm as usize]
                        .overflowing_shl(shift.unwrap_or_default() as u32)
                        .0,
                )
                .0,
        )),
        _ => return Err(error::invalid_operands()),
    }
}

/// Stores a byte, halfword, or word in memory
pub fn store_bytes(operands: &Operands, chip: &mut Processor, size: MemSize) -> Result<(), String> {
    let (rt, address) = get_rt_and_address(operands, chip)?;
    let address = error::check_memory_bounds(address, chip.memory.len(), size)?;
    let bytes = chip.R[rt as usize].to_le_bytes();
    for i in 0..size.bytes() {
        chip.memory[address + i] = bytes[i];
    }
    Ok(())
}

/// Stores a byte, halfword, or word in memory
pub fn load_bytes(operands: &Operands, chip: &mut Processor, size: MemSize) -> Result<(), String> {
    let (rt, address) = get_rt_and_address(operands, chip)?;
    let address = error::check_memory_bounds(address, chip.memory.len(), size)?;
    // store bytes, first element has lsb, value is zero extended
    let mut bytes: [u8; 4] = [0, 0, 0, 0];
    for i in 0..size.bytes() {
        // store least significant bits first
        bytes[i] = chip.memory[address + i];
    }
    chip.R[rt as usize] = u32::from_le_bytes(bytes);
    Ok(())
}

pub fn run_branch_instruction(
    std_out: &mut String,
    r0: u32,
    operands: &Operands,
    file_name: &str,
    line_number: usize,
    string_messages: &Vec<String>,
) -> Result<Option<InputStatus>, String> {
    match operands {
        Operands::label { label } => match label {
            Label::CR => {
                *std_out += "\n";
            }
            Label::VALUE => {
                *std_out = format!("{}{}", std_out, r0 as i32);
            }
            Label::PRINTCHAR => {
                *std_out = match char::from_u32(r0) {
                    Some(c) => format!("{}{}", std_out, c),
                    None => format!("{}Warning. Register value exceeds 255 and cannot be converted to an ascii character.", std_out),
                }
            }
            Label::PRINTF => {
                *std_out = format!(
                    "{}{}",
                    std_out, string_messages.get(r0 as usize).ok_or(format!("\"{}\" line {}: Cannot print string pointed to by register r0.", file_name, line_number))?
                );
            }
            Label::GetNumber => {
                return Ok(Some(InputStatus::GetNumber));
            }
            Label::GetChar => {
                return Ok(Some(InputStatus::GetChar));
            }
            Label::Index(_) => {}    // covered in execution function
        },
        _ => (),
    }
    Ok(None)
}
