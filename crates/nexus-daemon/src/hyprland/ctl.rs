use tokio::process::Command;

/// Performs a call to `hyrpctl` with the provided parameters: `hyprctl <flags> <command> <args>`
/// and returns the result as a string.
pub(crate) async fn hyprctl(
    flags: &[&str],
    command: &str,
    args: &[&str],
) -> anyhow::Result<String> {
    let output = Command::new("hyprctl")
        .args(flags)
        .arg(command)
        .args(args)
        .output()
        .await?;

    let output = String::from_utf8(output.stdout)?;
    Ok(output)
}

pub(crate) async fn hyprctl_eval(arg: &str) -> anyhow::Result<String> {
    hyprctl(&[], "eval", &[arg]).await
}
