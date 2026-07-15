use std::{
    collections::{HashMap, hash_map::Entry},
    path::PathBuf,
};

use nexusctl::nexus_api::config_dir;

const HISTORY_FILENAME: &str = "launcher.hist";
const HISTORY_LEN: usize = 20;

#[derive(Clone, Debug)]
pub struct History {
    file: PathBuf,
    records: Vec<String>,
    occurrences: HashMap<String, u32>,
}

impl Default for History {
    fn default() -> Self {
        Self {
            file: Self::default_path(),
            records: Default::default(),
            occurrences: Default::default(),
        }
    }
}

impl History {
    pub fn default_path() -> PathBuf {
        config_dir().join(HISTORY_FILENAME)
    }

    pub fn read_from(path: impl Into<PathBuf>) -> std::io::Result<Self> {
        let path = path.into();
        let content = std::fs::read_to_string(&path)?;
        let mut history = Self {
            file: path,
            ..Default::default()
        };

        for record in content.lines() {
            history.add_record(record.to_owned());
        }

        Ok(history)
    }

    pub fn add_record(&mut self, record: String) {
        self.records.push(record.clone());
        match self.occurrences.entry(record) {
            Entry::Occupied(mut occupied) => *occupied.get_mut() += 1,
            Entry::Vacant(vacant) => {
                vacant.insert(1);
            }
        }
    }

    pub fn get(&self, record: &str) -> u32 {
        self.occurrences.get(record).copied().unwrap_or_default()
    }

    pub fn write(&self) -> std::io::Result<()> {
        let start = self.records.len().saturating_sub(HISTORY_LEN);
        let lines = self.records[start..].join("\n");

        std::fs::write(&self.file, lines)
    }
}
