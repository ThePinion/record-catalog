use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Format {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Label {
    name: String,
    catno: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Identifier {
    #[serde(rename = "type")]
    type_: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    position: String,
    type_: String,
    title: String,
    duration: String,
    sub_tracks: Option<Vec<Track>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    id: i64,
    year: i32,
    title: String,
    formats: Vec<Format>,
    artists: Vec<Artist>,
    labels: Vec<Label>,
    identifiers: Vec<Identifier>,
    genres: Vec<String>,
    styles: Vec<String>,
    tracklist: Vec<Track>,
}
