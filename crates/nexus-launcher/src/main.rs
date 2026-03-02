mod desktop;
mod launcher;

use nexus_widgets::NexusWidgetRunner;

use crate::launcher::NexusLauncher;

fn main() {
    let launcher = NexusLauncher::new();
    NexusWidgetRunner::run(launcher).unwrap()
}
