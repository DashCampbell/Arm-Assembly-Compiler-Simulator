use regex::Regex;
use std::{collections::HashMap, str::FromStr, thread, time::Duration};
use tauri::State;

pub use crate::instructions::*;
use crate::{
    backend_api::{compile, GlobalKillSwitch, GlobalProcessor},
    error::{self, CompileErr, InstructionCompileErr},
    utils as hp,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemSize {
    BYTE,
    HALFWORD,
    WORD,
}
impl MemSize {
    pub fn bytes(&self) -> usize {
        match *self {
            MemSize::BYTE => 1,
            MemSize::HALFWORD => 2,
            MemSize::WORD => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Shifts applied to registers. Shifts an element by k bits, k should be <= 32.
/// TODO: Use this enum
pub enum Shift {
    // LSL(u8),
    // LSR(u8),
    // ASR(u8),
    // ROR(u8),
    // RRX,
}

#[derive(Debug, PartialEq)]
pub enum ITStatus {
    OUT,
    IN,
    LAST,
}
#[derive(Debug, PartialEq, serde::Serialize)]
pub enum DebugStatus {
    END, // no more instructions to run
    CONTINUE,
    BREAKPOINT,
    RUNNING,
}
#[derive(Debug, PartialEq, serde::Serialize)]
pub enum InputStatus {
    GetChar,
    GetNumber,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Label {
    Index(usize),
    CR,
    VALUE,
    PRINTF,
    PRINTCHAR,
    GetChar,
    GetNumber,
}
impl FromStr for Label {
    type Err = Vec<String>;

    fn from_str(label: &str) -> Result<Self, Self::Err> {
        match label {
            "cr" => Ok(Self::CR),
            "value" => Ok(Self::VALUE),
            "printchar" => Ok(Self::PRINTCHAR),
            "getchar" => Ok(Self::GetChar),
            "getnumber" => Ok(Self::GetNumber),
            "printf" => Ok(Self::PRINTF),
            _ => Err(CompileErr::message(format!(
                "Label \"{}\" may not exist.",
                label
            ))),
        }
    }
}

/// Contains all labels, and handles all label logic
#[derive(Debug)]
pub struct Labels {
    global_labels: HashMap<String, usize>,
    local_labels: HashMap<String, usize>,
}
impl Labels {
    fn new() -> Self {
        Self {
            global_labels: HashMap::new(),
            local_labels: HashMap::new(),
        }
    }
    pub fn get_global_labels(config: &compile::Config) -> Result<Self, Vec<String>> {
        let mut labels = Self::new();
        // The global PC index, used for labels.
        let mut pc = 0usize;
        let mut errors = CompileErr::new();
        let global_regex = Regex::new(r"\s*.global\s+\w+\s*").unwrap();

        for (file_name, file_content) in config.read_contents()? {
            errors.update_current_file(file_name.clone());
            // get all local labels first
            labels.get_local_labels(&file_content, &mut pc, &mut errors);

            // get all global directives in a file
            for mat in global_regex.find_iter(&file_content) {
                let line = compile::preprocess_line(mat.as_str());
                let words: Vec<&str> = line.split_whitespace().collect();
                // get the label name and index
                let (label, index) = (
                    words[1].to_string(),
                    *labels.local_labels.get(words[1]).ok_or_else(|| {
                        CompileErr::message(format!(
                            "Global label \"{}\" is not defined in the file \"{}\".",
                            words[1], file_name
                        ))
                    })?,
                );
                if let Some(_) = labels.global_labels.insert(label, index) {
                    return Err(CompileErr::message(format!("Global label \"{}\" was already defined, attempting to overwrite global label in file \"{}\".", words[1], file_name)));
                }
            }
        }
        labels.local_labels.clear();
        Ok(labels)
    }
    /// Retrieves all local labels inside a file, and returns local string variables
    pub fn get_local_labels(
        &mut self,
        file_content: &String,
        pc: &mut usize,
        errors: &mut CompileErr,
    ) -> (Vec<String>, HashMap<String, usize>) {
        let re_label = Regex::new(r"^[a-zA-Z_]+\w*\s*:$").unwrap();
        let mut local_labels: HashMap<String, usize> = HashMap::new();
        let mut last_label: Option<String> = None;
        // stores a list of string variables
        let mut strings: Vec<String> = Vec::new();
        // key -> label, value -> label's index in list of string variables
        let mut string_labels: HashMap<String, usize> = HashMap::new();

        for (line_number, line) in file_content.lines().enumerate() {
            errors.update_line_number(line_number); // update line number for error messages
            let line = compile::preprocess_line(line);

            // skip if white space, or directive, or IT instruction or directive
            if line.starts_with(".string") {
                let line = line.split_once('\"');
                if let Some((_, mes)) = line {
                    strings.push(mes[..mes.len() - 1].to_string());
                    if let Some(label) = &last_label {
                        string_labels.insert(label.clone(), strings.len() - 1);
                    } else {
                        errors.push_message(".string variable should have a label beforehand.");
                    }
                } else {
                    errors.push_message("Invalid syntax.");
                }
            }
            if line.is_empty() || line.to_lowercase().starts_with("it") || line.starts_with('.') {
                continue;
            }
            // If it is a label, store it in the Hashmap of local_labels.
            if line.ends_with(':') {
                if re_label.is_match(line) {
                    let label = line.trim_end_matches(':').to_string();
                    local_labels.insert(label.clone(), *pc);
                    last_label = Some(label);
                } else {
                    errors.push_message("Invalid label.");
                }
            } else {
                *pc += 1; // increment PC for each instruction.
            }
        }
        self.local_labels = local_labels;
        (strings, string_labels)
    }
    fn get(&self, label: &str) -> Result<Label, Vec<String>> {
        if self.global_labels.contains_key(label) {
            Ok(Label::Index(*self.global_labels.get(label).unwrap()))
        } else if self.local_labels.contains_key(label) {
            Ok(Label::Index(*self.local_labels.get(label).unwrap()))
        } else {
            Label::from_str(label)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types, non_snake_case)]
/// Instruction Sub-Category, named Category for convenience.
pub enum Operands {
    Rd_immed {
        Rd: u8,
        immed: u32,
    },
    Rd_Rm {
        Rd: u8,
        Rm: u8,
        shift: Option<Shift>,
    },
    Rd_Rn_immed {
        Rd: u8,
        Rn: u8,
        immed: u32,
    },
    Rd_Rn_Rm {
        Rd: u8,
        Rn: u8,
        Rm: u8,
        shift: Option<Shift>,
    },
    label {
        label: Label,
    },
    Rt_Rn_imm {
        Rt: u8,
        Rn: u8,
        imm: Option<i32>,
    },
    Rt_Rn_imm_post {
        Rt: u8,
        Rn: u8,
        imm: i32,
    },
    Rt_Rn_imm_pre {
        Rt: u8,
        Rn: u8,
        imm: i32,
    },
    Rt_Rn_Rm {
        Rt: u8,
        Rn: u8,
        Rm: u8,
        shift: Option<u8>,
    },
    Rt_label {
        Rt: u8,
        label: usize,
    },
    Rt_imm {
        Rt: u8,
        label: u32,
    },
}
impl Operands {
    pub fn is_memory_operands(line: &str) -> Result<Self, Vec<String>> {
        let mut errors = InstructionCompileErr::new();
        let operands = Operands::from_str(line)?;

        match operands {
            Operands::Rt_Rn_imm { .. }
            | Operands::Rt_Rn_imm_post { .. }
            | Operands::Rt_Rn_imm_pre { .. } => (),
            Operands::Rt_Rn_Rm { shift, .. } => {
                errors.check_mem_left_shift(shift);
            }
            _ => return Err(error::invalid_args(line)),
        }
        errors.result(operands)
    }
}
impl FromStr for Operands {
    type Err = Vec<String>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let args = hp::get_all_numbers(line)?;
        if hp::is_Rd_immed(line) {
            Ok(Self::Rd_immed {
                Rd: args[0] as u8,
                immed: args[1],
            })
        } else if hp::is_Rd_Rm(line) {
            Ok(Self::Rd_Rm {
                Rd: args[0] as u8,
                Rm: args[1] as u8,
                shift: None,
            })
        } else if hp::is_Rd_Rn_immed(line) {
            Ok(Self::Rd_Rn_immed {
                Rd: args[0] as u8,
                Rn: args[1] as u8,
                immed: args[2],
            })
        } else if hp::is_Rd_Rn_Rm(line) {
            Ok(Self::Rd_Rn_Rm {
                Rd: args[0] as u8,
                Rn: args[1] as u8,
                Rm: args[2] as u8,
                shift: None,
            })
        } else if hp::is_Rt_Rn(line) {
            Ok(Self::Rt_Rn_imm {
                Rt: args[0] as u8,
                Rn: args[1] as u8,
                imm: None,
            })
        } else if hp::is_Rt_Rn_imm(line) {
            Ok(Self::Rt_Rn_imm {
                Rt: args[0] as u8,
                Rn: args[1] as u8,
                imm: Some(args[2] as i32),
            })
        } else if hp::is_Rt_Rn_imm_post(line) {
            Ok(Self::Rt_Rn_imm_post {
                Rt: args[0] as u8,
                Rn: args[1] as u8,
                imm: args[2] as i32,
            })
        } else if hp::is_Rt_Rn_imm_pre(line) {
            Ok(Self::Rt_Rn_imm_pre {
                Rt: args[0] as u8,
                Rn: args[1] as u8,
                imm: args[2] as i32,
            })
        } else if hp::is_Rt_Rn_Rm(line) {
            Ok(Self::Rt_Rn_Rm {
                Rt: args[0] as u8,
                Rn: args[1] as u8,
                Rm: args[2] as u8,
                shift: None,
            })
        } else if hp::is_Rt_Rn_Rm_shift(line) {
            Ok(Self::Rt_Rn_Rm {
                Rt: args[0] as u8,
                Rn: args[1] as u8,
                Rm: args[2] as u8,
                shift: Some(args[3] as u8),
            })
        } else if hp::is_Rt_equal_label(line) {
            Ok(Self::Rt_label {
                Rt: args[0] as u8,
                label: 0,
            })
        } else if hp::is_Rt_equal_immed(line) {
            Ok(Self::Rt_imm {
                Rt: args[0] as u8,
                label: args[1],
            })
        } else {
            Err(error::invalid_args(line))
        }
    }
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
impl ConditionCode {
    #[allow(non_snake_case)]
    pub fn condition_test(&self, N: bool, Z: bool, C: bool, V: bool) -> bool {
        match *self {
            Self::EQ => Z,
            Self::NE => !Z,
            Self::CS => C,
            Self::CC => !C,
            Self::MI => N,
            Self::PL => !N,
            Self::VS => V,
            Self::VC => !V,
            Self::HI => C && !Z,
            Self::LS => !C || Z,
            Self::GE => (N && V) || (!N && !V),
            Self::LT => (N && !V) || (!N && V),
            Self::GT => !Z && ((N && V) || (!N && !V)),
            Self::LE => Z || (N && !V) || (!N && V),
            Self::AL => true,
        }
    }
    pub fn opposite_condition(&self) -> Self {
        match *self {
            Self::EQ => Self::NE,
            Self::NE => Self::EQ,
            Self::CS => Self::CC,
            Self::CC => Self::CS,
            Self::MI => Self::PL,
            Self::PL => Self::MI,
            Self::VS => Self::VC,
            Self::VC => Self::VS,
            Self::HI => Self::LS,
            Self::LS => Self::HI,
            Self::GE => Self::LT,
            Self::LT => Self::GE,
            Self::GT => Self::LE,
            Self::LE => Self::GT,
            Self::AL => Self::AL,
        }
    }
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
/// Metadata & Extensions that may be attached to mnemonics.
pub struct MnemonicExtension {
    pub cc: Option<ConditionCode>, // <cc> conditional code
    pub s: bool,                   // s flag
    pub w: bool,                   // .w extension
    pub it_status: ITStatus,       // in/out/last
}
impl MnemonicExtension {
    pub fn new() -> Self {
        MnemonicExtension {
            cc: None,
            s: false,
            w: false,
            it_status: ITStatus::OUT,
        }
    }
}
/// Contains an instruction line and metadata
struct Line {
    mnemonic: String,
    /// Used for error messages
    file_name: String,
    line_number: usize,
    extension: MnemonicExtension,
    is_breakpoint: bool,
    /// Used to run assembly code.
    operands: Operands,
}
impl Line {
    fn new(
        mnemonic: String,
        file_name: String,
        line_number: usize,
        extension: MnemonicExtension,
        is_breakpoint: bool,
        operands: Operands,
    ) -> Self {
        Line {
            mnemonic,
            file_name,
            line_number,
            extension,
            is_breakpoint,
            operands,
        }
    }
}

/// Contains the Assembly program.
/// Labels, lines, and a list of all Instructions
/// Initialized at compile time, cannot be changed at runtime.
pub struct Program {
    /// A list of compiled instruction lines
    lines: Vec<Line>,
    /// The Arm Intruction Set
    instructions: HashMap<String, Box<dyn Instruction>>,
    /// The delay between each instruction
    delay: u16,
    /// stores string variables
    pub string_messages: Vec<String>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            lines: Vec::new(),
            instructions: all_instructions(),
            delay: 0,
            string_messages: Vec::new(),
        }
    }
    pub fn reset(&mut self, delay: u16) {
        self.lines.clear();
        self.delay = delay;
        self.string_messages.clear();
    }
    /// Pushes a new compiled line.
    fn push_line(
        &mut self,
        mnemonic: String,
        file_name: &String,
        line_number: usize,
        extension: MnemonicExtension,
        is_breakpoint: bool,
        operands: Operands,
    ) {
        self.lines.push(Line::new(
            mnemonic,
            file_name.clone(),
            line_number,
            extension,
            is_breakpoint,
            operands,
        ));
    }
    /// Returns the mnemonic of a given line, if there is one.
    /// Warning: In implementation, the line is converted to lowercase first before being passed to this function.
    pub fn find_mnemonic(&self, line: &String) -> Option<(String, MnemonicExtension)> {
        // Get the first word.
        let mut line = line.split_whitespace().next()?;
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
    // Compiles a branch instruction
    /// Returns compile time errors, if instruction is invalid.
    fn compile_branch_instruction(
        &mut self,
        extension: &MnemonicExtension,
        line: &str,
        labels: &Labels,
    ) -> Result<Operands, Vec<String>> {
        if extension.s {
            return Err(CompileErr::message(
                "A branch instruction cannot have the S flag set.".into(),
            ));
        }
        // push compiled line onto instruction stack. Returns compile errors if any.
        if hp::is_label(line) {
            // get the string label
            let label = Regex::new(r"\w+$").unwrap().find(line).unwrap().as_str();
            // Validate label
            let operands = Operands::label {
                label: labels.get(label)?,
            };
            Ok(operands)
        } else {
            Err(CompileErr::message("Invalid branch instruction.".into()))
        }
    }
    fn compile_ldr_instruction(
        &mut self,
        _extension: &MnemonicExtension,
        line: &str,
        string_labels: &HashMap<String, usize>,
    ) -> Result<Operands, Vec<String>> {
        let mut errors = InstructionCompileErr::new();
        let operands = Operands::from_str(line)?;

        match operands {
            Operands::Rt_Rn_imm { .. }
            | Operands::Rt_Rn_imm_post { .. }
            | Operands::Rt_Rn_imm_pre { .. }
            | Operands::Rt_imm { .. } => (),
            Operands::Rt_Rn_Rm { shift, .. } => {
                errors.check_mem_left_shift(shift);
            }
            Operands::Rt_label { Rt, .. } => {
                // get the string label
                let label = Regex::new(r"\w+$").unwrap().find(line).unwrap().as_str();
                return Ok(Operands::Rt_label {
                    Rt,
                    label: *string_labels
                        .get(label)
                        .ok_or(InstructionCompileErr::invalid_label(label))?,
                });
            }
            _ => return Err(error::invalid_args(line)),
        }
        errors.result(operands)
    }
    /// Compiles an instruction.
    /// Returns compile time errors, if instruction is invalid.
    pub fn compile_instruction(
        &mut self,
        mnemonic: String,
        file_name: &String,
        line_number: usize,
        extension: MnemonicExtension,
        is_breakpoint: bool,
        original_line: &str,
        line: &String,
        labels: &Labels,
        string_labels: &HashMap<String, usize>,
    ) -> Result<(), Vec<String>> {
        // get instruction
        let instruction = self
            .instructions
            .get(&mnemonic)
            .expect("mnemonic should be valid.");

        // push compiled line onto instruction stack. Returns compile errors if any.
        let operands = if mnemonic == "b" || mnemonic == "bl" {
            // compile branch instructions separately.
            self.compile_branch_instruction(&extension, original_line, labels)?
        } else if mnemonic == "ldr" {
            self.compile_ldr_instruction(&extension, original_line, string_labels)?
        } else {
            instruction.get_operands(&extension, line)?
        };
        self.push_line(
            mnemonic,
            file_name,
            line_number,
            extension,
            is_breakpoint,
            operands,
        );

        Ok(())
    }
    /// Runs compiled assembly instuctions
    /// Returns Standard Output, or Standard Error message
    pub fn run(
        &self,
        processor: &mut Processor,
        shutdown: State<'_, GlobalKillSwitch>,
    ) -> Result<(String, InputStatus, DebugStatus), String> {
        // Starting at the PC index.
        let mut std_out = String::new();

        while (processor.R[15] as usize) < self.lines.len() {
            // get the line to run
            let line = &self.lines[processor.R[15] as usize];
            processor.R[15] += 1;

            let instruction = self
                .instructions
                .get(&line.mnemonic)
                .expect("run-time error, mnemonic should be valid!");
            // Compute condition code first.
            if let Some(cc) = line.extension.cc {
                // skip instruction if condition code not passed
                if !cc.condition_test(processor.N, processor.Z, processor.C, processor.V) {
                    continue;
                }
            }
            // handle predefined subroutines
            if line.mnemonic == "b" || line.mnemonic == "bl" {
                match line.operands {
                    Operands::label { label } => match label {
                        Label::CR => {
                            std_out += "\n";
                        }
                        Label::VALUE => {
                            std_out = format!("{}{}", std_out, processor.R[0] as i32);
                        }
                        Label::PRINTCHAR => {
                            std_out = match char::from_u32(processor.R[0]) {
                                Some(c) => format!("{}{}", std_out, c),
                                None => format!("{}Warning. Register value exceeds 255 and cannot be converted to an ascii character.", std_out),
                            }
                        }
                        Label::PRINTF => {
                            std_out = format!(
                                "{}{}",
                                std_out, self.string_messages.get(processor.R[0] as usize).ok_or(format!("\"{}\" line {}: Cannot print string pointed to by register r0.", line.file_name, line.line_number))?
                            );
                        }
                        Label::GetNumber => {
                            return Ok((std_out, InputStatus::GetNumber, DebugStatus::RUNNING));
                        }
                        Label::GetChar => {
                            return Ok((std_out, InputStatus::GetChar, DebugStatus::RUNNING));
                        }
                        Label::Index(_) => {}    // covered in execution function
                    },
                    _ => (),
                }
            }
            instruction
                .execute(line.extension.s, &line.operands, processor)
                .map_err(|err| {
                    format!("\"{}\" line {}: {}", line.file_name, line.line_number, err)
                })?;

            // shutdown program if Stop button was pressed.
            let mut kill_switch = shutdown.0.lock().expect("Error getting lock.");
            if *kill_switch {
                *kill_switch = false;
                drop(kill_switch);
                break;
            }
        }
        Ok((std_out, InputStatus::None, DebugStatus::END))
    }
    /// Debug compiled assembly instuctions
    /// If Ok, returns (current file name, current line number, debug status, Input Status, standard output)
    /// If Err, returns standard error
    pub fn debug_run(
        &self,
        processor: State<'_, GlobalProcessor>,
        shutdown: State<'_, GlobalKillSwitch>,
        std_input: Option<i32>,
    ) -> Result<(String, usize, DebugStatus, InputStatus, Option<String>), String> {
        // time delay for instruction
        thread::sleep(Duration::from_millis(self.delay as u64));

        // get processor
        let mut processor = processor
            .0
            .lock()
            .expect("Failed to get processor in run function.");
        if let Some(input) = std_input {
            processor.R[0] = input as u32;
        }
        // Terminate process if stop button was pressed, or end of file was reached.
        let mut kill_switch = shutdown.0.lock().expect("Error getting lock.");
        let mut std_out: String = String::new();

        if (processor.R[15] as usize) >= self.lines.len() || *kill_switch {
            *kill_switch = false;
            return Ok(("".into(), 0, DebugStatus::END, InputStatus::None, None));
        }
        // get the line to run
        let line = &self.lines[processor.R[15] as usize];
        let debug_status = if line.is_breakpoint {
            DebugStatus::BREAKPOINT
        } else {
            DebugStatus::CONTINUE
        };
        processor.R[15] += 1;
        let instruction = self
            .instructions
            .get(&line.mnemonic)
            .expect("run-time error, mnemonic should be valid!");
        // Compute condition code first.
        if let Some(cc) = line.extension.cc {
            // skip instruction if condition code not passed
            if !cc.condition_test(processor.N, processor.Z, processor.C, processor.V) {
                return Ok((
                    line.file_name.clone(),
                    line.line_number,
                    debug_status,
                    InputStatus::None,
                    None,
                ));
            }
        }
        // run line, if a run-time error occurs stop program.
        // handle predefined subroutines
        if line.mnemonic == "b" || line.mnemonic == "bl" {
            match line.operands {
                Operands::label { label } => match label {
                    Label::CR => {
                        std_out += "\n";
                    }
                    Label::VALUE => {
                        std_out = format!("{}{}", std_out, processor.R[0] as i32);
                    }
                    Label::PRINTCHAR => {
                        std_out = match char::from_u32(processor.R[0]) {
                            Some(c) => format!("{}{}", std_out, c),
                            None => format!("{}Warning. Register value exceeds 255 and cannot be converted to an ascii character.", std_out),
                        }
                    }
                    Label::PRINTF => {
                        std_out = format!(
                            "{}{}",
                            std_out, self.string_messages.get(processor.R[0] as usize).ok_or(format!("\"{}\" line {}: Cannot print string pointed to by register r0.", line.file_name, line.line_number))?
                        );
                    }
                    Label::GetNumber => {
                        return Ok((line.file_name.clone(), line.line_number,  debug_status, InputStatus::GetNumber, None));
                    }
                    Label::GetChar => {
                        return Ok((line.file_name.clone(), line.line_number,  debug_status, InputStatus::GetChar, None));
                    }
                    Label::Index(_) => {}    // covered in execution function
                },
                _ => (),
            }
        }
        instruction
            .execute(line.extension.s, &line.operands, &mut processor)
            .map_err(|runtime_error| {
                format!(
                    "\"{}\" line {}: {}",
                    line.file_name, line.line_number, runtime_error
                )
            })?;
        Ok((
            line.file_name.clone(),
            line.line_number,
            debug_status,
            InputStatus::None,
            if std_out.is_empty() {
                None
            } else {
                Some(std_out)
            },
        ))
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
/// Contains both CPU and Memory information.
pub struct Processor {
    /// SP = R[13], LR = R[14], PC = R[15]
    pub R: [u32; 16],
    pub N: bool,
    pub Z: bool,
    pub C: bool,
    pub V: bool,
    // size = 1kb = 1024 bytes
    // 1 byte = 8 bits
    /// RAM
    pub memory: [u8; 1024],
}
impl Processor {
    pub fn new() -> Self {
        // full descending stack
        let mut registers = [0; 16];
        registers[13] = 1023;
        Processor {
            R: registers,
            N: false,
            Z: false,
            C: false,
            V: false,
            memory: [0; 1024],
        }
    }
    /// Resets all values except the instructions hashmap.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
