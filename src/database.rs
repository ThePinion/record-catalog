use std::fs;

use crate::models::{error::Result, item_holder::ItemHolder, record::Record};

pub struct Database {
    file_path: String,
    pub data: Vec<ItemHolder>,
}

impl Database {
    pub fn new(file_path: &str) -> Result<Self> {
        Ok(Database {
            file_path: file_path.to_string(),
            data: Database::initial_load(file_path)?,
        })
    }

    pub fn contains(&self, record: &Record) -> bool {
        return self
            .data
            .iter()
            .map(|h| &h.record)
            .collect::<Vec<_>>()
            .contains(&record);
    }

    pub fn search(&self, query: &str) -> Vec<ItemHolder> {
        return self
            .data
            .iter()
            .filter(|r| {
                let json = serde_json::to_string(&r).unwrap().to_ascii_lowercase();
                return query
                    .to_ascii_lowercase()
                    .split_ascii_whitespace()
                    .all(|q| json.contains(q));
            })
            .map(|r| r.clone())
            .collect();
    }

    pub fn contains_id(&self, id: i64) -> bool {
        return self
            .data
            .iter()
            .map(|r| r.record.id)
            .collect::<Vec<_>>()
            .contains(&id);
    }

    pub fn add(&mut self, record: Record) -> Result<()> {
        if self.contains(&record) {
            return Ok(());
        }

        self.data.push(ItemHolder::new(record));

        self.save()
    }

    pub fn remove(&mut self, record: &Record, item_index: usize) -> Result<()> {
        let idx = self.data.iter().position(|x| &x.record == record);

        if let Some(index) = idx {
            let holder = &mut self.data[index];
            if holder.items.len() > item_index {
                holder.items.remove(item_index);
            }
            if holder.items.is_empty() {
                self.data.remove(index);
            }
        }

        Ok(())
    }

    fn initial_load(file_path: &str) -> Result<Vec<ItemHolder>> {
        let mut data = vec![];
        if std::path::Path::new(file_path).exists() {
            let data_string = fs::read_to_string(file_path)?;
            data = serde_json::from_str::<Vec<ItemHolder>>(&data_string)?;
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
