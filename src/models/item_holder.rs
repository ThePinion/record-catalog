use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::record::Record;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ItemHolder {
    pub record: Record,
    pub items: Vec<Item>,
}

impl ItemHolder {
    pub fn new(record: Record) -> Self {
        ItemHolder {
            record: record,
            items: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn add_item(&mut self) {
        self.items
            .push(Item::new_with_id(self.items.len().try_into().unwrap()))
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    pub id: i64,
    pub events: Vec<ItemEvent>,
}

impl Item {
    pub fn new_with_id(id: i64) -> Self {
        Item {
            id: id,
            events: vec![ItemEvent::new(ItemEventType::Created)],
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ItemEvent {
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub event_type: ItemEventType,
}

impl ItemEvent {
    pub fn new(event_type: ItemEventType) -> Self {
        ItemEvent {
            date: Utc::now(),
            event_type: event_type,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ItemEventType {
    Created,
    Deleted,
}
