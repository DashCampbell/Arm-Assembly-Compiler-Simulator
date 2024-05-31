// For Compiling, Running, and Debugging assembly code.
mod fc;
mod arm7;


/// Compile assembly code.
/// Returns a list of compile-time errors if there are any.
pub fn Compile() -> Result((), Vec<String>) {
    let mut errors = Vec::new<String>();
    // Load file contents from main.s
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
}