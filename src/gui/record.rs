use std::vec;

use crate::models::record::{Record, Track};

enum TM {
    Line(String),
    Module(Tab),
}

impl TM {
    fn line_with_title(title: &str, items: Vec<String>, delimiter: &str) -> TM {
        let mut out = "".to_string();
        out += &title;
        for i in items {
            out += &i;
            out += &delimiter
        }
        out = out.trim_end_matches(delimiter).to_string();
        TM::Line(out)
    }

    fn blank() -> TM {
        TM::Line("".to_string())
    }
}

struct Tab {
    pub children: Vec<TM>,
}

impl Tab {
    fn new(children: Vec<TM>) -> Self {
        Tab { children: children }
    }

    #[allow(dead_code)]
    fn single(child: TM) -> Self {
        Tab {
            children: vec![child],
        }
    }

    fn from_strings(children: Vec<String>) -> Self {
        Tab {
            children: children.into_iter().map(|s| TM::Line(s)).collect(),
        }
    }

    fn get_lines(self, depth: usize) -> Vec<String> {
        let lines: Vec<String> = self
            .children
            .into_iter()
            .map(|c| match c {
                TM::Line(l) => vec![l],
                TM::Module(t) => t.get_lines(depth + 1),
            })
            .flat_map(|v| v)
            .map(|s| Tab::get_indent(depth) + &s)
            .collect();

        lines
    }
    fn get_indent(depth: usize) -> String {
        let mut out: String = "".to_owned();
        for _ in 0..depth {
            out += "  ";
        }
        out
    }
}

impl Record {
    pub fn get_lines(&self) -> Vec<String> {
        let mut main_tab_module = Tab::new(vec![
            TM::blank(),
            TM::Line(self.title.clone()),
            TM::blank(),
            TM::Line("Year: ".to_owned() + &self.year.to_string()),
            TM::Line("Artists:".to_string()),
            TM::Module(Tab::from_strings(
                self.artists.iter().map(|a| a.name.clone()).collect(),
            )),
            TM::Line("Formats:".to_string()),
            TM::Module(Tab::from_strings(
                self.formats.iter().map(|a| a.name.clone()).collect(),
            )),
            TM::line_with_title("Genres: ", self.genres.clone(), " | "),
            TM::line_with_title("Styles: ", self.styles.clone(), " | "),
            TM::Line("Tracklist: ".to_string()),
            TM::blank(),
        ]);

        self.tracklist
            .iter()
            .map(|t| t.get_module())
            .for_each(|m| main_tab_module.children.push(m));

        main_tab_module.children.append(&mut vec![
            TM::blank(),
            TM::blank(),
            TM::Line(format!("Id: {:?}", self.id)),
        ]);

        main_tab_module.get_lines(1)
    }
}

impl Track {
    fn get_module(&self) -> TM {
        let position = match self.position.as_str() {
            "" => "".to_string(),
            _ => self.position.clone() + ".",
        };
        let duration = match self.duration.as_str() {
            "" => "".to_string(),
            _ => " [".to_string() + &self.duration + "] ",
        };
        let mut tab = Tab::new(vec![TM::Line(position + &duration + &self.title)]);
        if let Some(artists) = &self.extraartists {
            let artists_module = TM::Module(Tab::from_strings(
                artists.iter().map(|a| a.name.clone()).collect(),
            ));
            tab.children.push(artists_module);
            tab.children.push(TM::Line("".to_string()));
        }
        if let Some(sub_tracks) = &self.sub_tracks {
            sub_tracks
                .iter()
                .map(|s| s.get_module())
                .for_each(|m| tab.children.push(m));
            tab.children.push(TM::Line("".to_string()));
        };
        TM::Module(tab)
    }
}
