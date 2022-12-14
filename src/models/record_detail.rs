use super::{
    app::{AppNode, Navigation},
    record::Record,
};

pub struct RecordDetail {
    pub record: Option<Record>,
    pub is_saved: bool,
    pub message: String,
    pub back_instruction: Navigation,
}

impl RecordDetail {
    pub fn empty(back_instruction: Navigation) -> Self {
        let message = "No record".to_string();
        RecordDetail {
            record: None,
            is_saved: true,
            message: message,
            back_instruction,
        }
    }
}

impl AppNode for RecordDetail {
    fn navigation(&mut self, navigation: Navigation) {
        match navigation {
            Navigation::ViewRelease(record) => self.record = Some(record),
            _ => {}
        }
    }
}
