use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscogsSearchResultRelease {
    pub id: i64,
    #[serde(rename = "type")]
    result_type: String,
    pub title: String,
    format: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscogsSearchResultOther {
    id: i64,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DiscogsSearchResult {
    DiscogsSearchResultRelease(DiscogsSearchResultRelease),
    DiscogsSearchResultOther(DiscogsSearchResultOther),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscogsSearchResponse {
    // pagination: String,
    pub results: Vec<DiscogsSearchResult>,
}

impl DiscogsSearchResponse {
    pub fn get_releases(self) -> Vec<DiscogsSearchResultRelease> {
        self.results
            .into_iter()
            .filter_map(|r| match r {
                DiscogsSearchResult::DiscogsSearchResultRelease(o) => Some(o),
                DiscogsSearchResult::DiscogsSearchResultOther(_) => None,
            })
            .collect()
    }
}
