use std::io;

use tokio::{io::AsyncReadExt, process::Command};

/// Performs a call to `hyrpctl` with the provided parameters: `hyprctl <flags> <command> <args>`
/// and returns the result as a string.
pub(crate) async fn hyprctl(flags: &[&str], command: &str, args: &[&str]) -> io::Result<String> {
    let mut handle = Command::new("hyprctl")
        .args(flags)
        .arg(command)
        .args(args)
        .spawn()?;

    handle.wait().await?;
    if let Some(mut stdout) = handle.stdout {
        let mut out = String::new();
        let _ = stdout.read_to_string(&mut out).await?;
        Ok(out)
    } else {
        Ok(String::new())
    }
}

pub(crate) async fn hyprctl_eval(arg: &str) -> io::Result<String> {
    hyprctl(&[], "eval", &[arg]).await
}
