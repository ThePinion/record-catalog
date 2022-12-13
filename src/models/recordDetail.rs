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
    pub fn empty(backInstruction: Navigation) -> Self {
        let message = "No record".to_string();
        RecordDetail {
            record: None,
            is_saved: true,
            message: message,
            back_instruction: backInstruction,
        }
    }
}

impl AppNode for RecordDetail {}
