use regex::Regex;
use std::{collections::HashMap};

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Operands {
    pub Rd: u8,
    pub Rn: u8,
    pub immed: i64,
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
    pub fn new()->Self{
        Operands{
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
#[derive(Debug)]
#[derive(PartialEq)]
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
#[allow(non_snake_case)]
/// Contains both CPU and Memory information.
pub struct Processor {
    // i64 has range (-2^63 to 2^63-1)
    // Stores values (-2^31 to 2^31-1) or (0, 2^32) 
    pub R: [i64; 16],
    pub N: bool,
    pub Z: bool,
    pub C: bool,
    pub V: bool,
    /// PC register, stores the location of the next intruction. (File location & line number)
    pub PC: String,
    // size = 1kb = 1024 bytes
    // 1 byte = 8 bits
    // i16 to store unsigned and signed bytes
    /// RAM
    pub memory: [i16; 1024],
    // A hashmap of labels. key = Label, value = index in Compile Lines list
    pub labels: HashMap<String, usize>,
}
impl Processor {
    pub fn new() -> Self {
        Processor {
            R: [0; 16],
            N: false,
            Z: false,
            C: false,
            V: false,
            PC: "".into(),
            memory: [0; 1024],
            labels: HashMap::new(),
        }
    }
}

/// Regex expression for unsigned immediate values
/// ex: #0x12, #12, #0b1100
fn re_u_number() -> &'static str {
    r"#(0b[01]+|0x[A-Fa-f\d]+|\d+)"
}
fn re_is_bin(num: &str) -> bool {
    Regex::new(r"^#0b[01]+$")
    .unwrap()
    .is_match(num)
}
fn re_is_hex(num: &str) -> bool {
    Regex::new(r"^#0x[A-Fa-f\d]+$")
    .unwrap()
    .is_match(num)
}
fn re_is_dec(num: &str) -> bool {
    Regex::new(r"#\d+$")
    .unwrap()
    .is_match(num)
}

/// Collect all unsigned numbers in a line. Including register numbers, hexadecimal, binary, immediate values, etc..
fn re_get_all_numbers(line: &str) -> Vec<i64> {
    Regex::new(format!(r"(r\d+|{})", re_u_number()).as_str())
    .unwrap()
    .find_iter(line)
    .map(|mat| {
        let m = mat.as_str();
        if re_is_bin(m) {
            // Binary
            println!("{}", m);
            i64::from_str_radix(&m[3..], 2).expect("error converting binary number")
        } else if re_is_hex(m){
            // Hexadecimal
            i64::from_str_radix(&m[3..], 16).expect("error converting hexadecimal number")
        }else if re_is_dec(m){
            // Immediate Decimal Value
            (&m[1..]).parse::<i64>().expect("error converting decimal number")
        }else {
            // Register Value
            (&m[1..]).parse::<i64>().expect("error converting decimal number")
        }
    })
    .collect::<Vec<i64>>()
}
fn re_check_s_flag(mnemonic: &str, line: &str) -> bool {
    Regex::new(format!(r"^{}s", mnemonic).as_str())
    .unwrap()
    .is_match(line)
}

pub trait Instruction {
    fn mnemonic() -> &'static str;
    /// Determines & validates the encoding type for an instruction line. Returns an error if the instruction is invalid.
    /// Called at compile time
    fn get_encoding(line: &str)-> Result<(Encoding, Operands), String>;
    fn encode(encoding: &Encoding, operands: &Operands) -> String;
    /// Returns Ok() if instruction executed correctly, returns Err() if there is a runtime error.
    /// Called at runtime.
    fn execute(encoding: &Encoding, operands: &Operands, chip: &mut Processor) -> Result<(), String>;
}


// Implement Instructions
pub struct MOV;
impl Instruction for MOV{
    fn mnemonic() -> &'static str {
        "mov"
    }
    fn get_encoding(line: &str)-> Result<(Encoding, Operands), String> {
        let line = line.trim();
        let s_flag = re_check_s_flag(MOV::mnemonic(), line);

        // Remove mnemonic and flags first
        if let Some((_, line)) = line.split_once(' ') {
            let re_imm = Regex::new(format!(r"^r\d+\s*,\s*{}$", re_u_number()).as_str()).unwrap();   // move immediate
            let re_reg = Regex::new(r"^r\d+\s*,\s*r\d+$").unwrap();  // move register

            // Trim whitespace
            let line = line.trim();
            let mut operands = Operands::new();
            
            if re_imm.is_match(line){
                let encoding = if !s_flag {
                    Encoding::ImmT1
                }else{
                    Encoding::ImmT2
                };
                
                let args = re_get_all_numbers(line);
                let rd = args[0];
                let immed = args[1];
                
                if rd > 14 {
                    return Err(format!("R{} is invalid. Only registers R0 to R14 are allowed.", rd))
                }
                if immed > 16 {
                    return Err(format!("Immediate value {} is invalid.", immed))
                }
                operands.Rd = rd as u8;
                operands.immed = immed;
                
                Ok((encoding, operands))
            }else if re_reg.is_match(line) {
                let encoding = if !s_flag {
                    Encoding::RegT1
                }else{
                    Encoding::RegT2
                };               
                let args = re_get_all_numbers(line);
                let rd = args[0];
                let rn = args[1];
                // Determine Encoding
                if rd > 14 {
                    return Err(format!("R{} is invalid. Only registers R0 to R14 are allowed.", rd))
                }
                if rn > 14 {
                    return Err(format!("R{} is invalid. Only registers R0 to R14 are allowed.", rn))
                }
                operands.Rd = rd as u8;
                operands.Rn = rn as u8;
                
                Ok((encoding, operands))
            }
            else{
                Err(format!("Invalid Arguments for intruction '{}'", MOV::mnemonic()))
            }
        }else {
            Err(format!("Not enough arguments for instruction '{}'", MOV::mnemonic()))
        }
    }
    fn encode(encoding: &Encoding, operands: &Operands) -> String {
        match encoding {
            Encoding::ImmT1 => format!("0010 0{:03b} {:08b}", operands.Rd, operands.immed),
            _=> "".into()
        }
    }
    fn execute(encoding: &Encoding, operands: &Operands, chip: &mut Processor) -> Result<(), String> {
        match encoding {
            Encoding::ImmT1 | Encoding::ImmT2 => chip.R[operands.Rd as usize] = operands.immed,
            Encoding::RegT1 | Encoding::RegT2 => chip.R[operands.Rd as usize] = chip.R[operands.Rn as usize],
            _=> return Err("Encoding for MOV instruction is wrong.".into()),
        }
        Ok(())
    }
}