use std::io::Read;

use serde::Deserialize;

use super::models::{error::Result, query::DiscogsSearchResponse, record::Record};

pub struct DiscogsClient {
    token: String,
}

impl DiscogsClient {
    pub fn new(token: &str) -> Self {
        DiscogsClient {
            token: token.to_string(),
        }
    }
    fn discogs_request<T: for<'a> Deserialize<'a>>(url: &str) -> Result<T> {
        let mut res = reqwest::get(url)?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        Ok(serde_json::from_str::<T>(&body)?)
    }

    pub fn query(&self, query_: &str) -> Result<DiscogsSearchResponse> {
        let url = format!(
            "https://api.discogs.com/database/search?q={}&token={}",
            query_, self.token
        );
        Self::discogs_request(&url)
    }

    pub fn get_release(self, id: i64) -> Result<Record> {
        let url = format!("https://api.discogs.com/releases/{}", id);
        Self::discogs_request(&url)
    }
}
