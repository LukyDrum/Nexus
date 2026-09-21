use tokio::process::Command;

#[derive(Debug)]
pub enum ControlError {
    Io(std::io::Error),
}

/// Performs a call to `hyrpctl` with the provided parameters: `hyprctl <flags> <command> <args>`
/// and returns the result as a string.
pub async fn hyprctl(flags: &[&str], command: &str, args: &[&str]) -> Result<String, ControlError> {
    let output = Command::new("hyprctl")
        .args(flags)
        .arg(command)
        .args(args)
        .output()
        .await
        .map_err(ControlError::Io)?;

    let output = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(output)
}

pub async fn hyprctl_eval(arg: &str) -> Result<String, ControlError> {
    hyprctl(&[], "eval", &[arg]).await
}
