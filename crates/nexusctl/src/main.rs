use clap::Parser;
use nexusctl::{NexusCommand, send_command};

#[derive(Debug, clap::Parser)]
#[command(version, about)]
struct NexusCli {
    #[command(subcommand)]
    pub command: NexusCommand,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = NexusCli::parse();

    send_command(cli.command).await
}
