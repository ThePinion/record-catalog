use std::{error::Error, fs};

use crate::models::{error::Result, record::Record};

pub struct Database {
    file_path: String,
    pub data: Vec<Record>,
}

impl Database {
    pub fn new(file_path: &str) -> Result<Self> {
        Ok(Database {
            file_path: file_path.to_string(),
            data: Database::initial_load(file_path)?,
        })
    }

    pub fn contains(&self, record: &Record) -> bool {
        return self.data.contains(record);
    }

    pub fn add(&mut self, record: Record) -> Result<()> {
        if self.contains(&record) {
            return Err("")?;
        }

        self.data.push(record);

        self.save()
    }

    fn initial_load(file_path: &str) -> Result<Vec<Record>> {
        let mut data = vec![];
        if std::path::Path::new(file_path).exists() {
            let data_string = fs::read_to_string(file_path)?;
            data = serde_json::from_str::<Vec<Record>>(&data_string)?;
        } else {
            let data_string = serde_json::to_string(&data)?;
            fs::write(file_path, data_string)?;
        }
        return Ok(data);
    }

    fn save(&self) -> Result<()> {
        let data_string = serde_json::to_string(&self.data)?;
        fs::write(&self.file_path, data_string)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct FileContainedError(String);
