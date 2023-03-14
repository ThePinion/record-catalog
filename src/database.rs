use std::fs;

use crate::models::{
    error::Result,
    item_holder::{Item, ItemHolder},
    record::Record,
};

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

    #[allow(dead_code)]
    pub fn contains(&self, record: &Record) -> bool {
        return self
            .data
            .iter()
            .map(|h| &h.record)
            .collect::<Vec<_>>()
            .contains(&record);
    }

    pub fn search(&self, query: &str, item_holder: Option<Record>) -> Vec<ItemHolder> {
        let results = self.data.iter().filter(|r| {
            let json = serde_json::to_string(&r).unwrap().to_ascii_lowercase();
            query
                .to_ascii_lowercase()
                .split_ascii_whitespace()
                .all(|q| json.contains(q))
        });

        match item_holder {
            Some(record) => {
                let mut res: Vec<_> = results
                    .filter(|ih| ih.record.id != record.id)
                    .map(|ih| ih.clone())
                    .collect();
                res.insert(
                    0,
                    self.data
                        .iter()
                        .find(|ih| ih.record.id == record.id)
                        .map_or(ItemHolder::new(record), |ih| ih.clone()),
                );
                res
            }
            None => results.map(|i| i.clone()).collect(),
        }
    }

    pub fn contains_id(&self, id: i64) -> bool {
        return self
            .data
            .iter()
            .map(|r| r.record.id)
            .collect::<Vec<_>>()
            .contains(&id);
    }

    pub fn add(&mut self, record: Record) -> Result<ItemHolder> {
        let holder = match self
            .data
            .iter_mut()
            .find(move |ih| ih.record.id == record.id)
        {
            Some(item_holder) => {
                item_holder.add_item();
                item_holder.clone()
            }
            None => {
                let item_holder = ItemHolder::new_with_item(record);
                self.data.push(item_holder.clone());
                item_holder
            }
        };

        self.save()?;
        Ok(holder)
    }

    pub fn update_item(&mut self, record: &Record, item: Item) -> Result<()> {
        let old_item = self
            .data
            .iter_mut()
            .find(|ih| &ih.record == record)
            .ok_or("No item holder matching the current record")?
            .items
            .iter_mut()
            .find(|i| i.id == item.id)
            .ok_or("No item holder matching the item id")?;
        old_item.events = item.events;
        self.save()
    }

    pub fn remove_holder_item(&mut self, record: &Record, item_index: usize) -> Result<()> {
        let idx = self.data.iter().position(|x| &x.record == record);

        if let Some(index) = idx {
            let holder = &mut self.data[index];
            if holder.items.len() > item_index {
                holder.items.remove(item_index);
            }
        }
        self.save()
    }

    pub fn remove_holder(&mut self, record: &Record) -> Result<()> {
        let idx = self.data.iter().position(|x| &x.record == record);
        if let Some(index) = idx {
            self.data.remove(index);
        }
        self.save()
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
