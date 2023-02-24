use crate::inline_parser::parse_inline;
use std::collections::HashMap;
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
pub enum FarceElement {
    FDialogue(Dialogue),
    FSceneHeading(SceneHeading),
    FAction(String),
    FPageBreak,
}

impl FarceElement {
    fn emphasis<'a>(&'a self, s: &'a String) -> String {
        let result = parse_inline(s);
        match result {
            Ok(expressions) => expressions
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
                    self.emphasis(&dialogue.character_line_as_text()),
                    self.emphasis(&dialogue.text)
                )
            }
            Self::FAction(action) => {
                format!(
                    "<div class=\"element-action\">\n<p>{}</p>\n</div>\n\n",
                    self.emphasis(action)
                )
            }
            Self::FPageBreak => "<div class=\"element-pagebreak\"></div>\n\n".to_string(),
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
}

fn truncate_string(ss: &String, num_chars: Option<usize>) -> String {
    let mut s = ss.clone();
    let num_chars = num_chars.unwrap_or(20);
    if s.chars().count() > num_chars {
        s.truncate(s.chars().take(num_chars).map(|c| c.len_utf8()).sum());
        s.push_str("...");
    }
    s
}

#[cfg(test)]
mod tests;
