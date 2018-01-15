//!Poems houses the structs and implementations for deserializing and
//! storing Poem data retrieved from poetrydb API requests
use serde_json::{self, Value};

use util;

#[derive(Serialize, Deserialize, Debug)]
pub struct WorksList {
    pub titles: Vec<String>,
}

impl WorksList {
    pub fn new() -> WorksList {
        let default: WorksList = WorksList {
            titles: Vec::new(),
        };
        let list: WorksList = util::read_titles_from_file().unwrap_or(default);
        return list
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Poem {
    pub title: String,
    pub author: String,
    pub lines: Vec<String>,
    pub line_count: i64,
}
impl Poem {
    pub fn new() -> Poem {
        Poem {
            title: String::new(),
            author: String::new(),
            lines: Vec::new(),
            line_count: 0,
        }
    }
    pub fn from_value(mut self, json: &Value) -> Result<Poem, serde_json::Error> {
        if let Some(ref lines) = json.get("lines") {
            &self.get_lines(lines)?;
        } else {
            println!("no lines found!");
        }
        if let Some(ref author) = json.get("author") {
            &self.get_author(author)?;
        } else {
            println!("no author name found!");
        }
        if let Some(ref title) = json.get("title") {
            &self.get_title(title)?;
        } else {
            println!("no title found!");
        }
        if let Some(ref l_c) = json.get("linecount") {
            &self.get_count(l_c)?;
        } else {
            println!("no linecount found!");
        }
        return Ok((self));

    }
    fn get_lines(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.lines = serde_json::from_value(json.clone())?;
        return Ok((self.to_owned()));
    }
    fn get_author(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.author = serde_json::from_value(json.clone())?;
        return Ok((self.to_owned()));
    }
    fn get_title(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.title = serde_json::from_value(json.clone())?;
        return Ok((self.to_owned()));
    }
    fn get_count(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.line_count = serde_json::from_value(json.clone())?;
        return Ok((self.to_owned()));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorsList {
    pub authors: Vec<String>,
}
impl AuthorsList {
    pub fn new() -> AuthorsList {
        let default: AuthorsList = AuthorsList { 
            authors: Vec::new(),
        };
        let list: AuthorsList = util::read_authors_from_file().unwrap_or(default); 
        return list
        
        }
}

