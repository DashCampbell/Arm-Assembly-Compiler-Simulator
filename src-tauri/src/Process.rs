// For Compiling, Running, and Debugging assembly code.
use crate::arm7::Processor;
use regex::Regex;
use std::fs;
use std::sync::Mutex;
use tauri::State;

/// Contains the Processor that runs all assembly code
pub struct GlobalProcessor(pub Mutex<Processor>);

/// Compile assembly code.
/// Returns a list of compile-time errors if there are any.
#[tauri::command(rename_all = "snake_case")]
pub fn compile(processor: State<GlobalProcessor>, dir_path: &str) -> Result<(), Vec<String>> {
    /// Stores all compile time errors
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
    let mut processor = processor
        .0
        .lock()
        .expect("Failed to get processor in compile function.");
    // Reset values of Processor
    processor.reset();

    let re_comment = Regex::new(r"^(\s*\/\/)|^(\s*$)").unwrap();
    for (line_number, line) in contents.lines().enumerate() {
        // skip if white space or comment.
        if re_comment.is_match(line) {
            continue;
        }
        // Remove comments at the end of a line
        // TODO: change when implementing String values.
        let original_line = line;
        let (line, _) = line.split_once("//").unwrap_or((line, ""));
        let line = line.trim().to_lowercase();

        // identify mnemonic
        if let Some(mnemonic) = processor.find_mnemonic(&line) {
            // Mnemonic is valid.
            if let Err(error_messages) = processor.compile_instruction(&mnemonic, &line) {
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
    // For each line in file
    // Process line:
    // If white space or comment, then skip
    // Remove comments at the end of line.
    // If in IT block, check validity.
    // Trim line.
    // Identify Mnemonic.
    // Get encoding of mnemonic.
    // If error, send error to standard output. continue
    // Get encoding.
    // Send line and metadata to Compiled Lines struct.
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
