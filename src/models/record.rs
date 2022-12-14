use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Format {
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Label {
    pub name: String,
    pub catno: String,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Identifier {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Artist {
    pub name: String,
    pub id: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Track {
    pub position: String,
    pub type_: String,
    pub title: String,
    pub duration: String,
    pub sub_tracks: Option<Vec<Track>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Record {
    pub id: i64,
    pub year: i32,
    pub title: String,
    pub formats: Vec<Format>,
    pub artists: Vec<Artist>,
    pub labels: Vec<Label>,
    pub identifiers: Vec<Identifier>,
    pub genres: Vec<String>,
    pub styles: Vec<String>,
    pub tracklist: Vec<Track>,
}
