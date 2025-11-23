use crate::task::Task;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct Storage {
    file_path: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir()
            .context("Could not find home directory")?;
        
        let file_path = home_dir.join(".task-cli-data.json");
        
        Ok(Storage { file_path })
    }

    pub fn load_tasks(&self) -> Result<Vec<Task>> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.file_path)
            .context("Failed to read tasks file")?;

        let tasks: Vec<Task> = serde_json::from_str(&content)
            .context("Failed to parse tasks from JSON")?;

        Ok(tasks)
    }

    pub fn save_tasks(&self, tasks: &[Task]) -> Result<()> {
        let json = serde_json::to_string_pretty(tasks)
            .context("Failed to serialize tasks to JSON")?;

        fs::write(&self.file_path, json)
            .context("Failed to write tasks to file")?;

        Ok(())
    }

    pub fn get_next_id(&self) -> Result<u32> {
        let tasks = self.load_tasks()?;
        Ok(tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1)
    }
}
