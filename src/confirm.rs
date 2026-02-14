use std::io::{BufRead, Write};

/// Prompt the user for confirmation. Returns `true` if confirmed.
///
/// If `skip` is `true`, returns `true` immediately (for `--yes` flag).
/// Prints to stderr so stdout stays clean for piped output.
/// Defaults to deny (N) when the user presses Enter without input.
pub fn confirm(message: &str, skip: bool) -> bool {
    if skip {
        return true;
    }

    eprint!("{message} [y/N] ");
    std::io::stderr().flush().ok();

    let stdin = std::io::stdin();
    let mut line = String::new();
    if stdin.lock().read_line(&mut line).is_err() {
        return false;
    }

    matches!(line.trim().to_lowercase().as_str(), "y" | "yes")
}
