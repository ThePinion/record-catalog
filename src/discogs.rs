use std::io::Read;

use crate::models::query::DiscogsSearchResponse;

pub struct DiscogsClient {
    token: String,
}

impl DiscogsClient {
    pub fn new(token: &str) -> Self {
        DiscogsClient {
            token: token.to_string(),
        }
    }
    pub fn query(self, query_: &str) -> DiscogsSearchResponse {
        let mut res = reqwest::get(&format!(
            "https://api.discogs.com/database/search?q={}&token={}",
            query_, self.token
        ))
        .unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        serde_json::from_str::<DiscogsSearchResponse>(&body).unwrap()
    }
}
