use std::process::Command;

use crate::HyprsError;

/// Performs a call to `hyrpctl` with the provided parameters: `hyprctl <flags> <command> <args>`
/// and returns the result as a string.
pub fn hyprctl(flags: &[&str], command: &str, args: &[&str]) -> Result<String, HyprsError> {
    let output = Command::new("hyprctl")
        .args(flags)
        .arg(command)
        .args(args)
        .output()
        .map_err(HyprsError::Io)?;

    let output = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(output)
}
