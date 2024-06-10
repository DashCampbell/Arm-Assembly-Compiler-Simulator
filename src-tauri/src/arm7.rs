use regex::Regex;
use std::{collections::HashMap, str::FromStr};

use crate::helpers as hp;
pub use crate::instructions::*;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Operands {
    pub Rd: u8,
    pub Rn: u8,
    pub immed: u32,
    pub Rm: u8,
    pub label: usize, // stores the index of an instruction line
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
            label: 0,
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
/// Instruction Sub-Category, named Category for convenience.
pub enum Category {
    Immediate,
    Register,
    Default,
}

/// Condition Codes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConditionCode {
    EQ,
    NE,
    CS,
    CC,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
}

impl FromStr for ConditionCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eq" => Ok(ConditionCode::EQ),
            "ne" => Ok(ConditionCode::NE),
            "cs" | "hs" => Ok(ConditionCode::CS),
            "cc" | "lo" => Ok(ConditionCode::CC),
            "mi" => Ok(ConditionCode::MI),
            "pl" => Ok(ConditionCode::PL),
            "vs" => Ok(ConditionCode::VS),
            "vc" => Ok(ConditionCode::VC),
            "hi" => Ok(ConditionCode::HI),
            "ls" => Ok(ConditionCode::LS),
            "ge" => Ok(ConditionCode::GE),
            "lt" => Ok(ConditionCode::LT),
            "gt" => Ok(ConditionCode::GT),
            "le" => Ok(ConditionCode::LE),
            "al" => Ok(ConditionCode::AL),
            _ => Err(format!("{} is not a condition code", s)),
        }
    }
}

#[derive(Debug, PartialEq)]
/// Extensions that may be attached to mnemonics.
pub struct MnemonicExtension {
    pub cc: Option<ConditionCode>, // <cc> conditional code
    pub s: bool,                   // s flag
    pub w: bool,                   // .w extension
}
impl MnemonicExtension {
    pub fn new() -> Self {
        MnemonicExtension {
            cc: None,
            s: false,
            w: false,
        }
    }
}
/// Contains an instruction line and metadata
struct Line {
    /// Used for error messages
    line: String,
    mnemonic: String,
    extension: MnemonicExtension,
    /// Used to run line.
    category: Category,
    /// Used to run line.
    operands: Operands,
}
impl Line {
    fn new(
        mnemonic: String,
        line: String,
        extension: MnemonicExtension,
        category: Category,
        operands: Operands,
    ) -> Self {
        Line {
            mnemonic,
            line,
            extension,
            category,
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
        instructions.insert("add".into(), Box::new(ADD {}));

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
        extension: MnemonicExtension,
        category: Category,
        operands: Operands,
    ) {
        self.lines.push(Line::new(
            mnemonic.clone(),
            line.clone(),
            extension,
            category,
            operands,
        ));
    }
    /// Returns the mnemonic of a given line, if there is one.
    /// Warning: In implementation, the line is converted to lowercase first before being passed to this function.
    pub fn find_mnemonic(&self, line: &String) -> Option<(String, MnemonicExtension)> {
        // Get the first word.
        let mut line = match line.split_whitespace().next() {
            Some(line) => line,
            None => return None,
        };
        // contains metadata on extensions to mnemonic
        let mut extension = MnemonicExtension::new();

        // assume no extensions on mnemonic
        if self.instructions.contains_key(line) {
            return Some((line.to_string(), extension));
        }
        // check for .w extension
        if line.ends_with(".w") {
            line = &line[..line.len() - 2];
            extension.w = true;
            if self.instructions.contains_key(line) {
                return Some((line.to_string(), extension));
            }
        }
        // assume condition code extension
        let re_cc = Regex::new(hp::condition_codes()).unwrap();
        let cc = &line[line.len() - 2..];
        if line.len() > 2 && re_cc.is_match(cc) {
            let line = &line[..line.len() - 2];

            // only contains condition code
            if self.instructions.contains_key(line) {
                extension.cc = Some(ConditionCode::from_str(cc).unwrap());
                return Some((line.to_string(), extension));
            }
            // assume S flag is also set
            if line.ends_with('s') && self.instructions.contains_key(&line[..line.len() - 1]) {
                extension.cc = Some(ConditionCode::from_str(cc).unwrap());
                extension.s = true;
                return Some(((&line[..line.len() - 1]).to_string(), extension));
            }
        }
        // check the S flag and no <cc> is set.
        if line.len() > 1 && line.ends_with('s') {
            if self.instructions.contains_key(&line[..line.len() - 1]) {
                extension.s = true;
                return Some(((&line[..line.len() - 1]).to_string(), extension));
            }
        }
        None
    }
    /// Compiles an instruction.
    /// Returns compile time errors, if instruction is invalid.
    pub fn compile_instruction(
        &mut self,
        mnemonic: &String,
        extension: MnemonicExtension,
        line: &String,
    ) -> Result<(), Vec<String>> {
        // get instruction
        let instruction = self
            .instructions
            .get(mnemonic)
            .expect("mnemonic should be valid.");

        // push compiled line onto instruction stack. Returns compile errors if any.
        let (category, operands) = instruction.get_category(&extension, line)?;
        self.push_line(mnemonic, line, extension, category, operands);

        Ok(())
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

            // run line, if a run-time error occurs stop program.
            instruction.execute(line.extension.s, line.category, &line.operands, processor)?;

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
        *self = Self::new();
    }
}
