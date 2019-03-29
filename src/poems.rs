//!Poems houses the structs and implementations for deserializing and
//! storing Poem data retrieved from poetrydb API requests
use serde_json::{self, Value};
use textwrap::{Wrapper, termwidth, fill};
// use textwrap::wrap_iter;
use hyphenation::*;
use hyphenation;
use util::{get_authors, get_titles};
use colored::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct WorksList {
    pub titles: Vec<String>,
}

impl WorksList {
    pub fn new() -> WorksList {
        // let default: WorksList = WorksList { titles: Vec::new() };
        let list: WorksList = get_titles();
        return list;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Poem {
    pub title: String,
    pub author: String,
    pub lines: Vec<String>,
    pub linecount: i64,
}

impl Poem {
    pub fn new() -> Poem {
        Poem {
            title: String::new(),
            author: String::new(),
            lines: Vec::new(),
            linecount: 0,
        }
    }
    pub fn from_value(mut self, json: &Value) -> Result<Poem, serde_json::Error> {
        if let Some(lines) = json.get("lines") {
            &self.get_lines(&lines)?;
        println!("got lines!\n{}", &self.lines.join("\n").blue());
        
        } else {
            println!("no lines found!");
        }
        if let Some(author) = json.get("author") {
            &self.get_author(&author)?;
            println!("got author!");

        } else {
            // println!("no author name found!");
        }
        if let Some(title) = json.get("title") {
            &self.get_title(&title)?;
            println!("got title!");
        } else {
            // println!("no title found!");
        }
        if let Some(l_c) = json.get("linecount") {
            println!("{:?}", l_c.as_i64());
            &self.get_count(&l_c)?;
            println!("{}","got line count!".green());

        } else {
            println!("no linecount found!");
        }
        return Ok(self);

    }
    pub fn print(&self) -> Self {
        let corpus = hyphenation::load(Language::English_US).unwrap();
        let width = termwidth() - 12;
        let author = format!("  author: {}", self.author.purple());
        let title = format!(" a poem: \"{}\"", self.title);
        let poem = self.lines.join("\n");
        let wrapper = Wrapper::with_splitter(width, corpus)
            .break_words(true)
            .subsequent_indent("        ");

        println!("  |{:=<1$}|", "=", width + 6);
        for t_line in wrapper.wrap_iter(&title) {
            let t_fmt = format!("{}", fill(&t_line, width));
            for t_l in t_fmt.lines() {
                println!("  |  {:<1$}    |", &t_l, width);
            }
        }
        println!("  |{:-<1$}|", "-", width + 6);

        // for line in poem.lines().into_iter() {
        for line in wrapper.wrap_iter(&poem) {
            let formatted = format!("{}", fill(&line, width));
            // let formatted = 
            // println!("  |   {:<1$}   |", fill(&formatted, width), width);

            // let formatted = format!("{:<1$}", fill(&line, width-9), width-9);
            for line in formatted.lines() {
                println!("  |   {:<1$}   |", 
                &line.bright_green(), 
                width
                );
            }
        }
        println!("  |{:-<1$}|", "-", width + 6);
        println!("  |{:<1$}      |", fill(&author, width), width + 9);
        println!("  |{:=<1$}|", "=", width + 6);
        return self.to_owned();
    }
    fn get_lines(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.lines = serde_json::from_value(json.clone())?;
        println!("lines from json object = \n{}", &self.lines.join("\n"));
        return Ok(self.to_owned());
    }
    fn get_author(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.author = serde_json::from_value(json.clone())?;
        return Ok(self.to_owned());
    }
    fn get_title(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        self.title = serde_json::from_value(json.clone())?;
        return Ok(self.to_owned());
    }
    fn get_count(&mut self, json: &Value) -> Result<Self, serde_json::Error> {
        println!("getting linecount");
        self.linecount = self.lines.len() as i64;
        return Ok(self.to_owned());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorsList {
    pub authors: Vec<String>,
}
impl AuthorsList {
    pub fn new() -> AuthorsList {
        // let default: AuthorsList = AuthorsList { authors: Vec::new() };
        let list: AuthorsList = get_authors();
        return list;

    }
}
