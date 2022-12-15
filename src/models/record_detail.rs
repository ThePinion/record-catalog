use super::{app::Navigation, record::Record};

#[derive(Clone)]
pub struct RecordDetail {
    pub record: Option<Record>,
    pub is_saved: bool,
    pub message: String,
    pub back_instruction: Box<Navigation>,
}

impl RecordDetail {
    pub fn empty(back_instruction: Navigation) -> Self {
        let message = "No record".to_string();
        RecordDetail {
            record: None,
            is_saved: true,
            message: message,
            back_instruction: Box::new(back_instruction),
        }
    }

    pub fn new(back_instruction: Navigation, record: Record, is_saved: bool) -> Self {
        let mut out = RecordDetail {
            record: Some(record),
            is_saved: is_saved,
            message: "".to_string(),
            back_instruction: Box::new(back_instruction),
        };
        out.set_saved(is_saved);
        out
    }

    pub fn set_saved(&mut self, saved: bool) {
        self.is_saved = saved;
        self.message = match self.is_saved {
            true => "Viewing the saved record.",
            false => "This record's not been saved yet. Press '+' to save it.",
        }
        .to_string();
    }
}
