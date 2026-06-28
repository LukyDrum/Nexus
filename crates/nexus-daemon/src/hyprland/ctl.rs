use tokio::{io::AsyncReadExt, process::Command};

pub(crate) async fn ctl_eval(command: String) -> anyhow::Result<String> {
    let mut handle = Command::new("hyprctl").arg("eval").arg(command).spawn()?;

    handle.wait().await?;
    if let Some(mut stdout) = handle.stdout {
        let mut out = String::new();
        let _ = stdout.read_to_string(&mut out).await?;
        Ok(out)
    } else {
        Ok(String::new())
    }
}
