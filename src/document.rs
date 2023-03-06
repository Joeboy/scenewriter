use crate::inline_parser::parse_inline;
use crate::utils::truncate_string;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug)]
pub struct Dialogue {
    pub character_name: String,
    pub character_extensions: Vec<String>, // The bit in brackets after the character name, eg "WILL (V.O)"
    pub text: String,
}

impl Dialogue {
    pub fn character_line_as_text(&self) -> String {
        match self.character_extensions.len() {
            0 => self.character_name.to_string(),
            _ => {
                let extensions = self.character_extensions.join(") (");
                format!("{} ({})", self.character_name, extensions)
            }
        }
    }

    pub fn get_num_words(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

#[derive(Debug)]
pub struct SceneHeading {
    pub int_or_ext: String,
    pub text: String,
}

impl fmt::Display for Dialogue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.character_name,
            truncate_string(&self.text, None)
        )
    }
}

impl fmt::Display for SceneHeading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.int_or_ext,
            truncate_string(&self.text, None)
        )
    }
}

#[derive(Debug)]
pub struct Action {
    pub is_centered: bool,
    pub text: String,
}

impl Action {
    pub fn get_num_words(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

#[derive(Debug)]
pub enum FarceElement {
    FDialogue(Dialogue),
    FSceneHeading(SceneHeading),
    FAction(Action),
    FPageBreak,
}

impl FarceElement {
    fn html_emphasis<'a>(&'a self, s: &'a String) -> String {
        let result = parse_inline(s);
        match result {
            Ok((_remainder, expressions)) => expressions
                .iter()
                .map(|e| e.as_html())
                .collect::<Vec<String>>()
                .join(""),
            Err(e) => format!("{}", e),
        }
    }

    pub fn as_html<'a>(&'a self) -> String {
        match self {
            Self::FSceneHeading(scene_heading) => {
                let scene_heading_str =
                    format!("{}. {}", scene_heading.int_or_ext, scene_heading.text);
                format!(
                    "<div class=\"scene-heading\">\n<p>{}</p>\n</div>\n\n",
                    scene_heading_str
                )
            }
            Self::FDialogue(dialogue) => {
                format!(
                    "<div class=\"element-dialogue\">\n<p>{}</p>\n<p>{}</p>\n</div>\n\n",
                    self.html_emphasis(&dialogue.character_line_as_text()),
                    self.html_emphasis(&dialogue.text)
                )
            }
            Self::FAction(action) => {
                format!(
                    "<div class=\"element-action\">\n<p>{}</p>\n</div>\n\n",
                    self.html_emphasis(&action.text)
                )
            }
            Self::FPageBreak => "<div class=\"element-pagebreak\"></div>\n\n".to_string(),
        }
    }

    pub fn get_all_chars(&self) -> String {
        // Return a string of all chars used by the element, so we know which
        // glyphs need to be embedded in the PDF
        match self {
            Self::FSceneHeading(scene_heading) => scene_heading.text.to_string(),
            Self::FDialogue(dialogue) => {
                format!("{}{}", dialogue.character_line_as_text(), dialogue.text)
            }
            Self::FAction(action) => action.text.to_string(),
            Self::FPageBreak => String::new(),
        }
    }
}

#[derive(Debug)]
pub struct TitlePage {
    pub fields: HashMap<String, String>,
}

#[derive(Debug)]
pub struct FarceDocument {
    pub title_page: Option<TitlePage>,
    pub elements: Vec<FarceElement>,
}

impl FarceDocument {
    pub fn get_titlepage_field(&self, field_name: &str) -> Option<&String> {
        match &self.title_page {
            Some(title_page) => title_page.fields.get(field_name),
            None => None,
        }
    }

    pub fn get_title(&self) -> Option<&String> {
        self.get_titlepage_field("Title")
    }

    pub fn has_title_page(&self) -> bool {
        match &self.title_page {
            Some(_title_page) => true,
            None => false,
        }
    }

    pub fn get_all_chars(&self) -> Vec<char> {
        // Get all the chars that appear in the doc, so we know which glyphs
        // we need to embed in the PDF.
        // Maybe we should keep track of the bold / italic chars separately,
        // suspect it wouldn't make a huge difference though.
        let mut unique_chars = HashSet::new();
        for e in &self.elements {
            for c in e.get_all_chars().chars() {
                if c != '\n' {
                    unique_chars.insert(c);
                }
            }
        }
        // Some characters that apear in "boilerplate", but could conceivably
        // not appear in the "text":
        unique_chars.extend("INTEXT._ ()".chars());
        unique_chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests;
