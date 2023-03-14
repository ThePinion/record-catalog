use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{list::StatefulList, record::Record};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ItemHolder {
    pub record: Record,
    pub items: Vec<Item>,
}

pub struct StatefulItemHolder {
    pub record: Record,
    pub detail_offset: usize,
    pub list: StatefulList<StatefulItem>,
}

impl ItemHolder {
    pub fn new(record: Record) -> Self {
        ItemHolder {
            record: record,
            items: vec![],
        }
    }

    pub fn new_with_item(record: Record) -> Self {
        ItemHolder {
            record: record,
            items: vec![Item::new_with_id(0)],
        }
    }

    pub fn add_item(&mut self) {
        self.items.push(Item::new_with_id(
            self.items
                .iter()
                .map(|item| item.id)
                .reduce(|accum, item| if accum >= item { accum } else { item })
                .unwrap_or(0)
                + 1,
        ))
    }

    pub fn to_stateful(self) -> StatefulItemHolder {
        StatefulItemHolder {
            record: self.record,
            list: StatefulList::with_items(self.items.into_iter().map(Item::to_stateful).collect()),
            detail_offset: 0,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    pub id: i64,
    pub events: Vec<ItemEvent>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct StatefulItem {
    pub item: Item,
    pub input: Option<ItemEventType>,
}

impl Item {
    pub fn new_with_id(id: i64) -> Self {
        Item {
            id: id,
            events: vec![ItemEvent::new(ItemEventType::Created)],
        }
    }
    pub fn to_stateful(self) -> StatefulItem {
        StatefulItem {
            item: self,
            input: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ItemEvent {
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
    pub event_type: ItemEventType,
    pub message: Option<String>,
}

impl ItemEvent {
    pub fn new(event_type: ItemEventType) -> Self {
        ItemEvent {
            date: Utc::now(),
            event_type: event_type,
            message: None,
        }
    }

    pub fn with_message(event_type: ItemEventType, message: String) -> Self {
        ItemEvent {
            date: Utc::now(),
            event_type: event_type,
            message: Some(message),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ItemEventType {
    Created,
    Message,
    Lent,
}
