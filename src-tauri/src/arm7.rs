use crate::helpers as hp;
use regex::Regex;
use std::collections::HashMap;

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
    fn new() -> Self {
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
    /// Used for error messages
    line: String,
    mnemonic: String,
    /// Used to run line.
    encoding: Encoding,
    /// Used to run line.
    operands: Operands,
}
impl Line {
    fn new(mnemonic: String, line: String, encoding: Encoding, operands: Operands) -> Self {
        Line {
            mnemonic,
            line,
            encoding,
            operands,
        }
    }
}

/// Contains the Assembly program.
/// Labels, lines, and a list of all Instructions
/// Initialized at compile time, cannot be changed at runtime.
pub struct Program {
    /// A hashmap of labels. key = Label, value = index in Compile Lines list
    labels: HashMap<String, usize>,
    /// A list of compiled instruction lines
    lines: Vec<Line>,
    /// The Arm Intruction Set
    instructions: HashMap<String, Box<dyn Instruction>>,
}

impl Program {
    pub fn new() -> Self {
        let mut instructions: HashMap<String, Box<dyn Instruction>> = HashMap::new();
        instructions.insert("mov".into(), Box::new(MOV {}));

        Program {
            labels: HashMap::new(),
            lines: Vec::new(),
            instructions,
        }
    }
    pub fn reset(&mut self) {
        self.labels.clear();
        self.lines.clear();
    }
    /// Pushes a new compiled line.
    fn push_line(
        &mut self,
        mnemonic: &String,
        line: &String,
        encoding: Encoding,
        operands: Operands,
    ) {
        self.lines.push(Line::new(
            mnemonic.clone(),
            line.clone(),
            encoding,
            operands,
        ));
    }
    /// Returns the mnemonic of a given line, if there is one.
    pub fn find_mnemonic(&self, line: &String) -> Option<String> {
        // Get the first word.
        let mut line = match line.split_whitespace().next() {
            Some(line) => line,
            None => return None,
        };
        // NOTE: An instruction cannot have a condition code and s flag extension at the same time.

        // check for condition code extension
        let re_cc = Regex::new(hp::condition_codes()).unwrap();
        if line.len() > 2 && re_cc.is_match(&line[line.len() - 2..]) {
            // remove condition code if it exists
            let line = &line[..line.len() - 2];
            if self.instructions.contains_key(line) {
                return Some(line.to_string());
            }
        }
        // remove S flag if it exists
        if line.len() > 1 && line.ends_with('s') {
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
                let _encoding_format = instruction.encode(encoding, &operands);
                // push compiled line onto instruction stack.
                self.push_line(mnemonic, line, encoding, operands);

                Ok(())
            }
            Err(err) => Err(err),
        }
    }
    /// Runs compiled assembly instuctions
    pub fn run(&self, processor: &mut Processor) -> Result<String, String> {
        // Starting at the PC index.
        while processor.PC < self.lines.len() {
            // get the line to run
            let line = &self.lines[processor.PC];
            // get the instruction
            let instruction = self
                .instructions
                .get(&line.mnemonic)
                .expect("run-time error, mnemonic should be valid!");

            // run line
            if let Err(std_err) = instruction.execute(line.encoding, &line.operands, processor) {
                // immediately stop running if an error is detected
                return Err(std_err);
            }
            processor.PC += 1;
        }
        Ok("".into())
    }
}

#[derive(Debug)]
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
    pub memory: [u8; 1024],
}
impl Processor {
    pub fn new() -> Self {
        Processor {
            R: [0; 16],
            N: false,
            Z: false,
            C: false,
            V: false,
            PC: 0,
            memory: [0; 1024],
        }
    }
    /// Resets all values except the instructions hashmap.
    pub fn reset(&mut self) {
        println!("reset processor");
        *self = Self::new();
    }
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
#[derive(Clone)]
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
        let s_flag = hp::check_s_flag(self.mnemonic(), line);

        // Remove mnemonic and flags first
        if let Some((_, line)) = line.split_once(' ') {
            let re_imm = Regex::new(format!(r"^r\d+\s*,\s*{}$", hp::u_number()).as_str()).unwrap(); // move immediate
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
                match hp::get_all_numbers(line) {
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
                match hp::get_all_numbers(line) {
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
