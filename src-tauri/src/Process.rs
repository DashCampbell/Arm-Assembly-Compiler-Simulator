// For Compiling, Running, and Debugging assembly code.
use crate::arm7::{ConditionCode, Processor, Program};
use crate::error::CompileErr;
use compile::CPU;
use regex::Regex;
use std::collections::VecDeque;
use std::fs;
use std::sync::Mutex;
use tauri::State;

/// Contains the Processor that runs all assembly code
pub struct GlobalProcessor(pub Mutex<Processor>);
/// Contains all the assembly code and instructions.
pub struct GlobalProgram(pub Mutex<Program>);

/// Compile assembly code.
/// Returns a list of compile-time errors if there are any.
#[tauri::command(rename_all = "snake_case")]
pub fn compile(
    processor: State<GlobalProcessor>,
    program: State<GlobalProgram>,
    dir_path: &str,
) -> Result<(), Vec<String>> {
    // Stores all compile time errors
    let mut errors = CompileErr::new();

    // Load file contents from main.s
    let contents = match fs::read_to_string(format!("{}main.s", dir_path)) {
        Ok(file) => file,
        Err(_) => {
            return Err(CompileErr::message(format!(
                "Couldn't find main.s file in directory {}",
                dir_path
            )));
        }
    };
    let mut processor = processor
        .0
        .lock()
        .expect("Failed to get processor in compile function.");
    // Reset CPU and Memory of Processor
    processor.reset();
    drop(processor);

    let mut program = program
        .0
        .lock()
        .expect("Failed to get program in compile function");
    // Reset compiled lines & labels of the program
    program.reset();

    // find all labels first
    let labels = compile::get_all_labels(&contents, &mut errors);

    // Represents the IT block. A list of Condition Codes representing If-Else conditions.
    let mut it_block: VecDeque<ConditionCode> = VecDeque::with_capacity(4);

    // Parse instructions
    for (line_number, line) in contents.lines().enumerate() {
        errors.update_line_number(line_number);
        let line = compile::preprocess_line(line).to_lowercase();

        // skip if white space or label
        if line.is_empty() || line.ends_with(':') {
            continue;
        }
        // Because an IT instruction affects future instructions, if there is an error in the IT statement
        // we cannot verify the correctness of the instructions within an IT block, so we return the compile
        // errors immediately.
        // Handle IT statement
        if compile::is_IT_statement(&line) {
            (it_block, errors) = compile::handle_IT_instruction(it_block, &line, errors)?;
        }
        // Handle other instructions.
        else if let Some((mnemonic, extension)) = program.find_mnemonic(&line) {
            // Mnemonic is valid.
            // if in IT block, validate condition code
            if let Some(correct_cc) = it_block.pop_front() {
                // check condition code
                if let Some(cc) = extension.cc {
                    if cc != correct_cc {
                        errors.push_message("The condition code must be the same or opposite of the IT block's condition code.");
                    }
                } else {
                    errors
                        .push_message("Instruction inside an IT block must have a condition code.");
                }
            }
            // handle branch instructions separately
            if mnemonic == "b" || mnemonic == "bl" {
                if let Err(err) =
                    program.compile_branch_instruction(&mnemonic, extension, &line, &labels)
                {
                    errors.extend(err);
                }
            } else if let Err(err) = program.compile_instruction(&mnemonic, extension, &line) {
                // return any compile time errors for this instruction.
                errors.extend(err);
            }
        } else {
            // No mnemonic detected.
            errors.push_message("Invalid instruction.");
        };
    }
    if !it_block.is_empty() {
        errors.push_message("IT block does not have all conditions covered.");
    }
    errors.result()
}

#[tauri::command(rename_all = "snake_case")]
/// Runs assembly code. Starting at the current PC index.
/// Returns Standard Output or Standard Error
pub fn run(
    processor: State<GlobalProcessor>,
    program: State<GlobalProgram>,
) -> Result<String, String> {
    // program is compiled and now immutable
    let program = program
        .0
        .lock()
        .expect("Failed to get Program in run function.");

    // get processor
    let mut processor = processor
        .0
        .lock()
        .expect("Failed to get processor in run function.");

    program.run(&mut processor)
}

#[tauri::command(rename_all = "snake_case")]
/// Sends CPU data to Frontend
pub fn display_CPU(processor: State<GlobalProcessor>, num_format: String) -> CPU {
    // get processor
    let processor = processor
        .0
        .lock()
        .expect("Failed to get processor in display_CPU function.");

    // format based on chosen number system.
    let formatter = match num_format.as_str() {
        "signed" => |r: u32| format!("{}", r as i32),
        "binary" => |r| format!("{:#034b}", r),
        "hexadecimal" => |r| format!("{:#010x}", r),
        _ => |r| format!("{}", r), // default is unsigned u32
    };
    let mut registers: Vec<String> = processor.R.into_iter().map(formatter).collect();
    // Add Link Register and Program Counter
    registers.push(formatter(processor.LR as u32));
    registers.push(formatter(processor.PC as u32));

    CPU {
        R: registers,
        N: processor.N,
        Z: processor.Z,
        C: processor.C,
        V: processor.V,
    }
}

#[tauri::command(rename_all = "snake_case")]
/// Sends Memory data to Frontend
pub fn display_Memory(processor: State<GlobalProcessor>, num_format: String) -> Vec<String> {
    // get processor
    let processor = processor
        .0
        .lock()
        .expect("Failed to get processor in display_CPU function.");

    // format based on chosen number system. Uses 8 bit representation.
    let formatter = match num_format.as_str() {
        "signed" => |byte: u8| format!("{}", byte as i8),
        "binary" => |byte| format!("{:#010b}", byte),
        "hexadecimal" => |byte| format!("{:#04x}", byte),
        _ => |byte| format!("{}", byte), // default is unsigned u32
    };
    processor.memory.into_iter().map(formatter).collect()
}

/// Contains all functions & structs pertaining to compiling assembly code.
mod compile {
    use super::{CompileErr, Regex, VecDeque};
    use crate::arm7::ConditionCode;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[derive(serde::Serialize)]
    #[allow(non_snake_case)]
    /// Used to output CPU data to Frontend
    pub struct CPU {
        pub R: Vec<String>,
        pub N: bool,
        pub Z: bool,
        pub C: bool,
        pub V: bool,
    }
    /// Removes comments, trims whitespace
    pub fn preprocess_line(line: &str) -> &str {
        let (line, _) = line.split_once("//").unwrap_or((line, "")); // Remove comments at the end of a line
        line.trim() // trim white space
    }
    pub fn is_IT_statement(line: &str) -> bool {
        Regex::new(r"^it[te]+\s+\w+$").unwrap().is_match(line)
    }
    pub fn get_all_labels<'a>(
        content: &'a String,
        errors: &mut CompileErr,
    ) -> HashMap<&'a str, usize> {
        let re_label = Regex::new(r"^[a-zA-Z_]+\w*\s*:$").unwrap();
        let mut pc = 0usize;

        let mut labels: HashMap<&str, usize> = HashMap::new();
        for (line_number, line) in content.lines().enumerate() {
            errors.update_line_number(line_number); // update line number for error messages
            let line = preprocess_line(line);
            // If it is a label, store it in the Hashmap of labels.
            if line.ends_with(':') {
                if re_label.is_match(line) {
                    labels.insert(line.trim_end_matches(':'), pc);
                } else {
                    errors.push_message("Invalid label.")
                }
            } else {
                pc += 1; // increment PC for each instruction.
            }
        }
        labels
    }
    pub fn handle_IT_instruction(
        mut it_block: VecDeque<ConditionCode>,
        line: &String,
        mut errors: CompileErr,
    ) -> Result<(VecDeque<ConditionCode>, CompileErr), Vec<String>> {
        // check if IT block is within another IT block
        if !it_block.is_empty() {
            errors.push_message("IT statement cannot be inside another IT block.");
            return Err(errors.early_return().unwrap_err());
        }
        // split line into IT<x<y<z>>> and condition code.
        let line = line.split_whitespace().collect::<Vec<&str>>();

        // get the default condition statement
        let default_cc = if let Some(cc) = line.get(1) {
            match ConditionCode::from_str(cc) {
                Ok(cc) => cc,
                Err(err) => {
                    // Cancel compilation if no valid condition code is given.
                    // Cannot validate condition code for future instructions inside the IT block,
                    // if no valid condition code is given.
                    errors.push_message(err.as_str());
                    return Err(errors.early_return().unwrap_err());
                }
            }
        } else {
            errors.push_message("IT statement must have a base condition.");
            return Err(errors.early_return().unwrap_err());
        };
        // get the list of if else conditions
        for (index, c) in line[0][1..].chars().enumerate() {
            if index > 4 {
                errors.push_message("An IT statement can only have conditions for 4 instructions.");
            }
            if c == 't' {
                it_block.push_back(default_cc);
            } else {
                // c == 'e'
                it_block.push_back(default_cc.opposite_condition());
            }
        }
        Ok((it_block, errors))
    }
}
