use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Settings {
    pub discogs_key: String,
    pub database_path: String,
}
