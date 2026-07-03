use clap::Parser;

use crate::{
    NexusWidgetRunner,
    quick::{args::QuickArgs, widget::QuickWidget},
};

mod args;
mod widget;

pub fn parse_and_run() -> anyhow::Result<()> {
    let args = QuickArgs::parse();
    let widget = QuickWidget::try_from(args)?;

    NexusWidgetRunner::run(widget)?;

    Ok(())
}
