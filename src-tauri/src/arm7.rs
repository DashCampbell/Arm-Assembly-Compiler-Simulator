use regex::Regex;
use std::collections::HashMap;
use std::mem;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Operands {
    pub Rd: u8,
    pub Rn: u8,
    pub immed: u32,
    pub Rm: u8,
    pub label: String,
    pub shift: String,
    pub rotation: u8,
    pub lsb: u8,
    pub width: u8,
    pub registers: Vec<u8>,
    pub Rd_lo: u8,
    pub Rd_hi: u8,
}
impl Operands {
    pub fn new() -> Self {
        Operands {
            Rd: 0,
            Rn: 0,
            immed: 0,
            Rm: 0,
            label: "".into(),
            shift: "".into(),
            rotation: 0,
            lsb: 0,
            width: 0,
            registers: Vec::new(),
            Rd_lo: 0,
            Rd_hi: 0,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
/// Instructions normally have sperate encodings for immediate and register operands
pub enum Encoding {
    // Immediate Encoding type
    ImmT1,
    ImmT2,
    ImmT3,
    ImmT4,
    // Register Encoding type
    RegT1,
    RegT2,
    RegT3,
    RegT4,
}

/// Contains an instruction line and metadata
struct Line {
    line: String,
    encoding: Encoding,
    operands: Operands,
}
impl Line {
    fn new(line: String, encoding: Encoding, operands: Operands) -> Self {
        Line {
            line,
            encoding,
            operands,
        }
    }
}

#[allow(non_snake_case)]
/// Contains both CPU and Memory information.
pub struct Processor {
    pub R: [u32; 16],
    pub N: bool,
    pub Z: bool,
    pub C: bool,
    pub V: bool,
    /// PC register, stores the index of the next intruction.
    PC: usize,
    // size = 1kb = 1024 bytes
    // 1 byte = 8 bits
    /// RAM
    memory: [u8; 1024],
    /// A hashmap of labels. key = Label, value = index in Compile Lines list
    labels: HashMap<String, usize>,
    /// A list of compiled instruction lines
    lines: Vec<Line>,
    /// The Arm Intruction Set
    instructions: HashMap<String, Box<dyn Instruction>>,
}
impl Processor {
    pub fn new() -> Self {
        let mut instructions: HashMap<String, Box<dyn Instruction>> = HashMap::new();
        instructions.insert("mov".into(), Box::new(MOV {}));

        Processor {
            R: [0; 16],
            N: false,
            Z: false,
            C: false,
            V: false,
            PC: 0,
            memory: [0; 1024],
            labels: HashMap::new(),
            lines: Vec::new(),
            instructions,
        }
    }
    /// Resets all values except the instructions hashmap.
    pub fn reset(&mut self) {
        let mut instructions: HashMap<String, Box<dyn Instruction>> = HashMap::new();
        mem::swap(&mut self.instructions, &mut instructions);
        *self = Processor {
            instructions,
            ..Self::new()
        }
    }
    /// Pushes a new compiled line.
    pub fn push_line(&mut self, line: &str, encoding: Encoding, operands: Operands) {
        self.lines
            .push(Line::new(line.to_string(), encoding, operands));
    }
    /// Returns the mnemonic of a given line, if there is one.
    pub fn find_mnemonic(&self, line: &String) -> Option<String> {
        // Get the first word.
        let mut line = match line.split_whitespace().next() {
            Some(line) => line,
            None => return None,
        };
        // remove condition code if it exists
        let re_cc = Regex::new(re_condition_codes()).unwrap();
        if line.len() > 2 && re_cc.is_match(&line[line.len() - 2..]) {
            line = &line[..line.len() - 2];
        }
        // remove S flag if it exists
        if line.len() > 1 && line.chars().last().unwrap() == 's' {
            line = &line[..line.len() - 1];
        }
        if self.instructions.contains_key(line) {
            Some(line.to_string())
        } else {
            None
        }
    }
    /// Compiles an instruction.
    /// Returns compile time errors, if instruction is invalid.
    pub fn compile_instruction(
        &mut self,
        mnemonic: &String,
        line: &String,
    ) -> Result<(), Vec<String>> {
        // get instruction
        let instruction = self
            .instructions
            .get(mnemonic)
            .expect("mnemonic should be valid.");

        match instruction.get_encoding(line) {
            Ok((encoding, operands)) => {
                // TODO: do something with the encoding format.
                let encoding_format = instruction.encode(encoding, &operands);
                // push compiled line onto instruction stack.
                self.push_line(line, encoding, operands);

                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

/// Regex expression for every condition code.
fn re_condition_codes() -> &'static str {
    r"(eq|ne|cs|hs|cc|lo|mi|pl|vs|vc|hi|ls|ge|lt|gt|le|al)"
}
/// Regex expression for unsigned immediate values
/// ex: #0x12, #12, #0b1100
fn re_u_number() -> &'static str {
    r"#(0b[01]+|0x[A-Fa-f\d]+|\d+)"
}
/// Regex expression for signed immediate values
/// ex: #0x12, #-12, #-0b1100, #12
fn re_i_number() -> &'static str {
    r"#-?(0b[01]+|0x[A-Fa-f\d]+|\d+)"
}
fn re_is_bin(num: &str) -> bool {
    Regex::new(r"^#-?0b[01]+$").unwrap().is_match(num)
}
fn re_is_hex(num: &str) -> bool {
    Regex::new(r"^#-?0x[A-Fa-f\d]+$").unwrap().is_match(num)
}
fn re_is_dec(num: &str) -> bool {
    Regex::new(r"#-?\d+$").unwrap().is_match(num)
}

/// Collect all unsigned/signed numbers in a line. Including register numbers, hexadecimal, binary, immediate values, etc..
/// Returns an error if there are invalid registers, numbers are invalid, out of bounds, etc...
fn re_get_all_numbers(line: &str) -> Result<Vec<i64>, Vec<String>> {
    let mut errors: Vec<String> = Vec::new();
    let mut numbers: Vec<i64> = Vec::new();

    for mat in Regex::new(format!(r"(r\d+|{})", re_i_number()).as_str())
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
        } else {
            // handle immediate values
            // check for negative value
            let mut sign: i64 = 1;
            let mut index_offset = 0; // if there is a minus sign, offset the index by one.
            if mat.starts_with("#-") {
                sign = -1;
                index_offset = 1;
            }
            let num = if re_is_bin(mat) {
                i64::from_str_radix(&mat[3 + index_offset..], 2)
            } else if re_is_hex(mat) {
                // Hexadecimal
                i64::from_str_radix(&mat[3 + index_offset..], 16)
            } else {
                // Immediate Decimal Value
                (&mat[1 + index_offset..]).parse::<i64>()
            };
            // check for out of bounds error
            if let Ok(num) = num {
                // acceptable values are -2^31 to 2^32 - 1 inclusive
                const LOWER_BOUND: i64 = -(2 as i64).pow(31);
                const UPPER_BOUND: i64 = (2 as i64).pow(32) - 1;

                if LOWER_BOUND <= num && num <= UPPER_BOUND {
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
fn re_check_s_flag(mnemonic: &str, line: &str) -> bool {
    Regex::new(format!(r"^{}s", mnemonic).as_str())
        .unwrap()
        .is_match(line)
}

pub trait Instruction: Send + Sync {
    fn mnemonic(&self) -> &'static str;
    fn valid_encodings(&self) -> &'static str;
    /// Determines & validates the encoding type and operands for an instruction line. Returns an error if the instruction is invalid.
    /// Called at compile time
    fn get_encoding(&self, line: &str) -> Result<(Encoding, Operands), Vec<String>>;
    // Encodes an instruction
    fn encode(&self, encoding: Encoding, operands: &Operands) -> String;
    /// Returns Ok() if instruction executed correctly, returns Err() if there is a runtime error.
    /// Called at runtime.
    fn execute(
        &self,
        encoding: Encoding,
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
    fn valid_encodings(&self) -> &'static str {
        "mov rd, rn\n
        mov rd, #immed"
    }
    fn get_encoding(&self, line: &str) -> Result<(Encoding, Operands), Vec<String>> {
        let line = line.trim();
        let s_flag = re_check_s_flag(self.mnemonic(), line);

        // Remove mnemonic and flags first
        if let Some((_, line)) = line.split_once(' ') {
            let re_imm = Regex::new(format!(r"^r\d+\s*,\s*{}$", re_u_number()).as_str()).unwrap(); // move immediate
            let re_reg = Regex::new(r"^r\d+\s*,\s*r\d+$").unwrap(); // move register

            // Trim whitespace
            let line = line.trim();
            let mut operands = Operands::new();

            if re_imm.is_match(line) {
                let encoding = if !s_flag {
                    Encoding::ImmT1
                } else {
                    Encoding::ImmT2
                };
                match re_get_all_numbers(line) {
                    Ok(args) => {
                        let rd = args[0];
                        let immed = args[1];

                        operands.Rd = rd as u8;
                        operands.immed = immed as u32;

                        Ok((encoding, operands))
                    }
                    Err(errors) => Err(errors),
                }
            } else if re_reg.is_match(line) {
                let encoding = if !s_flag {
                    Encoding::RegT1
                } else {
                    Encoding::RegT2
                };
                match re_get_all_numbers(line) {
                    Ok(args) => {
                        let rd = args[0];
                        let rn = args[1];

                        operands.Rd = rd as u8;
                        operands.Rn = rn as u8;

                        Ok((encoding, operands))
                    }
                    Err(errors) => Err(errors),
                }
            } else {
                Err(vec![format!(
                    "Invalid Arguments for intruction '{}'",
                    self.mnemonic()
                )])
            }
        } else {
            Err(vec![format!(
                "Not enough arguments for instruction '{}'",
                self.mnemonic()
            )])
        }
    }
    fn encode(&self, encoding: Encoding, operands: &Operands) -> String {
        match encoding {
            Encoding::ImmT1 => format!("0010 0{:03b} {:08b}", operands.Rd, operands.immed),
            _ => "".into(),
        }
    }
    fn execute(
        &self,
        encoding: Encoding,
        operands: &Operands,
        chip: &mut Processor,
    ) -> Result<(), String> {
        match encoding {
            Encoding::ImmT1 | Encoding::ImmT2 => chip.R[operands.Rd as usize] = operands.immed,
            Encoding::RegT1 | Encoding::RegT2 => {
                chip.R[operands.Rd as usize] = chip.R[operands.Rn as usize]
            }
            _ => return Err("Encoding for MOV instruction is wrong.".into()),
        }
        Ok(())
    }
}
