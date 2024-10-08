// For Compiling, Running, and Debugging assembly code.
use crate::arm7::{ConditionCode, DebugStatus, InputStatus, Labels, Processor, Program};
use crate::error::CompileErr;
use compile::{Config, CPU};
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use tauri::State;

/// Contains the Processor that runs all assembly code
pub struct GlobalProcessor(pub Mutex<Processor>);
/// Contains all the assembly code and instructions.
pub struct GlobalProgram(pub Mutex<Program>);
/// A kill switch to terminate the program while running.
pub struct GlobalKillSwitch(pub Mutex<bool>);

/// Compile assembly code.
/// Returns a list of compile-time errors if there are any.
#[tauri::command(rename_all = "snake_case")]
pub async fn compile(
    processor: State<'_, GlobalProcessor>,
    program: State<'_, GlobalProgram>,
    kill_switch: State<'_, GlobalKillSwitch>,
    dir_path: &str,
    breakpoint_map: Option<HashMap<&str, Vec<usize>>>,
) -> Result<(), Vec<String>> {
    // Load file contents, and get time delay
    let config = Config::new(dir_path)?;

    let mut processor = processor
        .0
        .lock()
        .expect("Failed to get processor in compile function.");
    // Reset CPU and Memory of Processor
    processor.reset();
    drop(processor);
    // reset kill switch
    *kill_switch.0.lock().unwrap() = false;

    let mut program = program
        .0
        .lock()
        .expect("Failed to get program in compile function");
    // Reset compiled lines & labels of the program
    program.reset(config.get_delay());

    // Represents the IT block. A list of Condition Codes representing If-Else conditions.
    let mut it_block: VecDeque<ConditionCode> = VecDeque::with_capacity(4);
    // The global PC index, used for labels.
    let mut pc = 0usize;
    // Stores all compile time errors
    let mut errors = CompileErr::new();
    // Stores all local and global labels
    let mut labels = Labels::get_global_labels(&config)?;
    // Stores labels that refer to a string variable. *string variables are stored in a vector that is needed at runtime.
    let mut string_labels: HashMap<String, usize> = HashMap::new();

    // Compile each file
    for (file_name, file_content) in config.read_contents()? {
        errors.update_current_file(file_name.clone());
        it_block.clear();

        // find all labels first
        let (new_strings, new_string_labels) =
            labels.get_local_labels(&file_content, &mut pc, &mut errors);
        string_labels.extend(
            new_string_labels
                .into_iter()
                .map(|(k, v)| (k, v + program.string_messages.len())),
        );
        program.string_messages.extend(new_strings);

        // Parse instructions
        for (line_number, line) in file_content.lines().enumerate() {
            let line_number = line_number + 1; // offset line number by one
            let original_line = compile::preprocess_line(line);
            let line = original_line.to_lowercase(); // set entire line to lowercase for easier parsing
            let is_breakpoint = breakpoint_map.as_ref().map_or(false, |map| {
                map.get(file_name.as_str())
                    .map_or(false, |list| list.contains(&line_number))
            });
            errors.update_line_number(line_number);

            // skip if white space or label or directive
            if line.is_empty() || line.ends_with(':') || line.starts_with('.') {
                continue;
            }
            // Handle IT statement
            if compile::is_if_then_block(&line) {
                errors = errors.handle_it_instruction(&mut it_block, line)?;
            }
            // Handle other instructions.
            else if let Some((mnemonic, mut extension)) = program.find_mnemonic(&line) {
                // Valid Mnemonic.
                extension.it_status = errors.get_it_status(&mut it_block, extension.cc);
                // return any compile time errors for this instruction.
                if let Err(err) = program.compile_instruction(
                    mnemonic,
                    file_name,
                    line_number,
                    extension,
                    is_breakpoint,
                    original_line,
                    &line,
                    &labels,
                    &string_labels,
                ) {
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
    }
    errors.result()
}

#[tauri::command(rename_all = "snake_case")]
/// Runs assembly code. Starting at the current PC index.
/// Returns Standard Output or Standard Error
pub async fn run(
    processor: State<'_, GlobalProcessor>,
    program: State<'_, GlobalProgram>,
    kill_switch: State<'_, GlobalKillSwitch>,
    std_input: Option<i32>,
) -> Result<(String, InputStatus, DebugStatus), String> {
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
    if let Some(input) = std_input {
        processor.R[0] = input as u32;
    }
    program.run(&mut processor, kill_switch)
}

#[tauri::command(rename_all = "snake_case")]
/// Returns Standard Output or Standard Error
/// If Ok, returns (current file name, current line number, Debug status, Input Status, standard output)
pub async fn debug_run(
    processor: State<'_, GlobalProcessor>,
    program: State<'_, GlobalProgram>,
    kill_switch: State<'_, GlobalKillSwitch>,
    std_input: Option<i32>,
) -> Result<(String, usize, DebugStatus, InputStatus, Option<String>), String> {
    // program is compiled and now immutable
    let program = program
        .0
        .lock()
        .expect("Failed to get Program in run function.");

    program.debug_run(processor, kill_switch, std_input)
}
#[tauri::command]
/// Stops the current assembly code from running.
pub async fn kill_process(kill_switch: State<'_, GlobalKillSwitch>) -> Result<(), ()> {
    let mut switch = kill_switch
        .0
        .lock()
        .expect("Kill switch lock was poisoned.");
    *switch = true;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
/// Sends CPU data to Frontend
pub async fn display_cpu(
    processor: State<'_, GlobalProcessor>,
    num_format: String,
) -> Result<CPU, ()> {
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
    let registers: Vec<String> = processor.R.into_iter().map(formatter).collect();
    let (n, z, c, v) = (processor.N, processor.Z, processor.C, processor.V);
    Ok(CPU {
        R: registers,
        N: n,
        Z: z,
        C: c,
        V: v,
    })
}

#[tauri::command(rename_all = "snake_case")]
/// Sends Memory data to Frontend
pub async fn display_memory(
    processor: State<'_, GlobalProcessor>,
    num_format: String,
) -> Result<(Vec<String>, u32), ()> {
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
    Ok((
        processor.memory.into_iter().map(formatter).collect(),
        processor.R[13],
    ))
}

/// Contains all functions & structs pertaining to compiling assembly code.
pub mod compile {
    use super::{CompileErr, Regex};
    use std::fs;

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
    pub fn read_dir_file(dir_path: &str, file_name: &String) -> Result<String, Vec<String>> {
        match fs::read_to_string(format!("{}{}", dir_path, file_name)) {
            // If there is no config file, default to reading from main.s
            Ok(file) => Ok(file),
            Err(_) => {
                return Err(CompileErr::message(format!(
                    "Couldn't find the file \"{}\" in directory: {}",
                    file_name, dir_path
                )));
            }
        }
    }
    #[derive(serde::Deserialize, Debug)]
    pub struct Config<'a> {
        /// A list of file names in the current directory.
        files: Vec<String>,
        #[serde(default)]
        delay: u16,
        #[serde(skip)]
        dir_path: &'a str,
    }
    impl<'a> Config<'a> {
        /// Creates the configuration for the compiler. Gets all files that will be compiled and the time delay for instructions.
        pub fn new(dir_path: &'a str) -> Result<Self, Vec<String>> {
            let config = match fs::read_to_string(format!("{}config.json", dir_path)) {
                // parse config file
                Ok(content) => serde_json::from_str::<Config>(&content).map_err(|err| {
                    CompileErr::message(format!("Configuration Error in \"config.json\" {}", err))
                }),
                // Default configuration.
                Err(_) => Ok(Self {
                    files: vec!["main.s".into()],
                    delay: 0,
                    dir_path: "",
                }),
            };
            config.map(|mut config| {
                config.dir_path = dir_path;
                config
            })
        }
        /// get time delay
        pub fn get_delay(&self) -> u16 {
            self.delay
        }
        /// Returns an iterator over all the file contents. A list of (file_name, file_content) for each file.
        pub fn read_contents(&self) -> Result<Vec<(&String, String)>, Vec<String>> {
            let mut iterator: Vec<(&String, String)> = Vec::new();
            for file_name in &self.files {
                let file_content = read_dir_file(self.dir_path, &file_name)?;
                iterator.push((file_name, file_content));
            }
            Ok(iterator)
        }
    }
    /// Removes comments & trims whitespace
    pub fn preprocess_line(line: &str) -> &str {
        let (line, _) = line.split_once("//").unwrap_or((line, "")); // Remove comments at the end of a line
        line.trim() // trim white space
    }
    pub fn is_if_then_block(line: &str) -> bool {
        Regex::new(r"^it[te]*\s+\w+$").unwrap().is_match(line)
    }
}
