use std::{
    collections::HashMap,
    fs::{self, read_dir},
    io::{self},
    process::{Child, Command},
};

use xdg::BaseDirectories;

const PERCENT_ENCODED_CODES: [&str; 5] = ["%F", "%u", "%U", "%i", "%"];
const APPLICATIONS_DIR: &str = "applications";

#[derive(Clone, Debug)]
pub struct DesktopEntry {
    name: String,
    exec: String,
}

impl DesktopEntry {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn run(&self) -> io::Result<Child> {
        let exec = PERCENT_ENCODED_CODES
            .iter()
            .fold(self.exec.clone(), |exec, code| exec.replace(code, ""));

        Command::new(exec.trim()).spawn()
    }
}

pub(crate) fn read_desktop_entries() -> Vec<DesktopEntry> {
    let base_dirs = BaseDirectories::new();

    let name_to_exec = base_dirs
        .get_data_dirs()
        .into_iter()
        .map(|dir| dir.join(APPLICATIONS_DIR))
        .filter_map(|dir| read_dir(dir).ok())
        .flatten()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let content = fs::read_to_string(path).ok()?;
            extract_name_exec(&content)
        })
        .collect::<HashMap<_, _>>();

    name_to_exec
        .into_iter()
        .map(|(name, exec)| DesktopEntry { name, exec })
        .collect()
}

fn extract_name_exec(text: &str) -> Option<(String, String)> {
    const DELIMETER: char = '=';
    const NAME_KEY: &str = "Name";
    const EXEC_KEY: &str = "Exec";
    const NO_DISPLAY_KEY: &str = "NoDisplay";
    const TRUE: &str = "true";

    let mut name = None;
    let mut exec = None;

    for line in text.lines() {
        let Some((key, value)) = line.split_once(DELIMETER) else {
            continue;
        };
        if key == NAME_KEY {
            name = Some(name.unwrap_or(value));
        } else if key == EXEC_KEY {
            exec = Some(exec.unwrap_or(value));
        } else if key == NO_DISPLAY_KEY && value.starts_with(TRUE) {
            return None;
        }
    }

    name.and_then(|name| exec.map(|exec| (name.to_owned(), exec.to_owned())))
}
