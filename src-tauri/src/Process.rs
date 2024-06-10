// For Compiling, Running, and Debugging assembly code.
use crate::arm7::{Processor, Program};
use regex::Regex;
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
    let mut errors: Vec<String> = Vec::new();

    // Load file contents from main.s
    let contents = match fs::read_to_string(format!("{}main.s", dir_path)) {
        Ok(file) => file,
        Err(_) => {
            errors.push(format!(
                "Couldn't find main.s file in directory {}",
                dir_path
            ));
            return Err(errors);
        }
    };
    // Reset CPU and Memory of Processor
    let mut processor = processor
        .0
        .lock()
        .expect("Failed to get processor in compile function.");
    processor.reset();
    drop(processor);

    let mut program = program
        .0
        .lock()
        .expect("Failed to get program in compile function");
    // Reset compiled lines & labels of the program
    program.reset();

    let re_comment = Regex::new(r"^(\s*\/\/)|^(\s*$)").unwrap();
    for (line_number, line) in contents.lines().enumerate() {
        // skip if white space or comment.
        if re_comment.is_match(line) {
            continue;
        }
        // Remove comments at the end of a line
        // TODO: change when implementing String variables.
        let original_line = line; // used for error messages
        let (line, _) = line.split_once("//").unwrap_or((line, ""));
        let line = line.trim().to_lowercase();

        // identify mnemonic
        if let Some((mnemonic, extension)) = program.find_mnemonic(&line) {
            // Mnemonic is valid.
            if let Err(error_messages) = program.compile_instruction(&mnemonic, extension, &line) {
                // return any compile time errors for this instruction.
                for e in error_messages {
                    errors.push(format!("Line {}: {}", line_number + 1, e));
                }
            }
        } else {
            // No mnemonic detected.
            errors.push(format!(
                "Line {}: \"{}\" is not an arm instruction.",
                line_number + 1,
                original_line
            ));
        };
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Runs assembly code. Starting at the current PC index.
/// Returns a list of run-time errors if there are any.
#[tauri::command(rename_all = "snake_case")]
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

#[derive(serde::Serialize)]
#[allow(non_snake_case)]
pub struct CPU {
    R: Vec<String>,
    N: bool,
    Z: bool,
    C: bool,
    V: bool,
}

#[tauri::command(rename_all = "snake_case")]
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
    let R: Vec<String> = processor.R.into_iter().map(formatter).collect();

    CPU {
        R,
        N: processor.N,
        Z: processor.Z,
        C: processor.C,
        V: processor.V,
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn display_Memory(processor: State<GlobalProcessor>, num_format: String) -> Vec<String> {
    // get processor
    let processor = processor
        .0
        .lock()
        .expect("Failed to get processor in display_CPU function.");

    // format based on chosen number system.
    let formatter = match num_format.as_str() {
        "signed" => |byte: u8| format!("{}", byte as i8),
        "binary" => |byte| format!("{:#010b}", byte),
        "hexadecimal" => |byte| format!("{:#04x}", byte),
        _ => |byte| format!("{}", byte), // default is unsigned u32
    };
    processor.memory.into_iter().map(formatter).collect()
}
