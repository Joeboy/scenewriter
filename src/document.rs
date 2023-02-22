use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while, take_while1, take_while_m_n},
    character::complete::{char, line_ending, multispace0, not_line_ending, space0, space1},
    combinator::{map, opt},
    multi::many1,
    sequence::{delimited, pair, separated_pair, terminated},
    IResult,
};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Dialogue {
    pub character_name: String,
    pub character_extension: Option<String>, // The bit in brackets after the character name, eg "WILL (V.O)"
    pub text: String,
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
    pub fn as_html(&self) -> String {
        match self {
            Self::FSceneHeading(scene_heading) => {
                format!(
                    "<div class=\"scene-heading\">\n<p>{}. {}</p></div>",
                    scene_heading.int_or_ext, scene_heading.text
                )
            }
            Self::FDialogue(dialogue) => {
                format!(
                    "<div class=\"element-dialogue\">\n<p>{}</p><p>{}</p></div>",
                    dialogue.character_name, dialogue.text
                )
            }
            Self::FAction(action) => {
                format!("<div class=\"element-action\">\n<p>{}</p></div>", action)
            }
            Self::FPageBreak => "<div class=\"element-pagebreak\"></div>".to_string(),
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

fn is_character_name_char(c: char) -> bool {
    // For now let's say speaker names can only have caps and spaces
    c.is_ascii_uppercase() || c == ' ' || c.is_ascii_digit()
}

fn parse_character_name(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    let (i, name) = take_while(|c: char| is_character_name_char(c))(input)?;
    let (i, extension) = opt(delimited(tag("("), is_not(")"), tag(")")))(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, (name, extension)))
}

fn one_or_more_non_newline_chars(input: &str) -> IResult<&str, &str> {
    let (i, line) = take_while1(|c: char| c != '\r' && c != '\n')(input)?;
    Ok((i, line))
}

fn nonempty_line(input: &str) -> IResult<&str, &str> {
    //let (i, line) = take_while1(|c: char| c != '\r' && c != '\n')(input)?;
    let (i, line) = one_or_more_non_newline_chars(input)?;
    let (i, _) = line_ending(i)?;
    Ok((i, line))
}

fn consume_whitespace(input: &str) -> IResult<&str, &str> {
    let (i, whitespace) = take_while(|c: char| c == '\n')(input)?;
    Ok((i, whitespace))
}

fn parse_scene_heading(input: &str) -> IResult<&str, FarceElement> {
    // Like EXT. A field in England
    map(
        pair(
            terminated(alt((tag("INT"), tag("EXT"))), char('.')),
            delimited(multispace0, not_line_ending, line_ending),
        ),
        |s: (&str, &str)| {
            FarceElement::FSceneHeading(SceneHeading {
                int_or_ext: s.0.to_string(),
                text: s.1.to_string(),
            })
        },
    )(input)
}

fn parse_dialogue(input: &str) -> IResult<&str, FarceElement> {
    let (i, (character_name, extension)) = parse_character_name(input)?;
    let (remainder, lines) = many1(nonempty_line)(i)?;
    let e = FarceElement::FDialogue(Dialogue {
        character_name: String::from(character_name),
        character_extension: {
            if let Some(character_extension) = extension {
                Some(String::from(character_extension))
            } else {
                None
            }
        },
        text: String::from(lines.join(" ")),
    });
    let (remainder, _) = consume_whitespace(remainder)?;
    Ok((remainder, e))
}

fn parse_action(input: &str) -> IResult<&str, FarceElement> {
    let (remainder, lines) = many1(terminated(
        take_while1(|c| c != '\r' && c != '\n'), // Not sure why not_newline doesn't work here?
        line_ending,
    ))(input)?;
    let (remainder, _) = consume_whitespace(remainder)?;
    Ok((remainder, FarceElement::FAction(lines.join("\n"))))
}

fn parse_page_break(input: &str) -> IResult<&str, FarceElement> {
    let (remainder, _) =
        terminated(take_while_m_n(3, 1e23 as usize, |s| s == '='), line_ending)(input)?;
    Ok((remainder, FarceElement::FPageBreak))
}

fn parse_element(input: &str) -> IResult<&str, FarceElement> {
    let (remainder, element) = alt((
        parse_scene_heading,
        parse_dialogue,
        parse_page_break,
        parse_action,
    ))(input)?;
    let (remainder, _) = consume_whitespace(remainder)?;
    Ok((remainder, element))
}

pub fn parse_elements(input: &str) -> IResult<&str, Vec<FarceElement>> {
    many1(parse_element)(input)
}

fn parse_multiline_titlepage_field_key(input: &str) -> IResult<&str, &str> {
    // Matches the start of a multiline title page field, eg "Author:\n"
    terminated(
        is_not(":"),
        terminated(tag(":"), terminated(space0, line_ending)),
    )(input)
}

fn parse_multiline_titlepage_field(input: &str) -> IResult<&str, (String, String)> {
    let (remainder, (key, lines)) = pair(
        parse_multiline_titlepage_field_key,
        many1(delimited(space1, not_line_ending, line_ending)),
    )(input)?;

    Ok((remainder, (key.to_string(), lines.join("\n"))))
}

fn parse_simple_titlepage_field(input: &str) -> IResult<&str, (String, String)> {
    // Simple one-line field like "Author: John July"
    let (remainder, (key, value)) = terminated(
        separated_pair(is_not(":"), tag(": "), not_line_ending),
        line_ending,
    )(input)?;
    Ok((remainder, (key.to_string(), value.to_string())))
}

fn parse_titlepage_field(input: &str) -> IResult<&str, (String, String)> {
    let (remainder, (k, v)) = alt((
        parse_simple_titlepage_field,
        parse_multiline_titlepage_field,
    ))(input)?;
    Ok((remainder, (k, v)))
}

fn parse_title_page(input: &str) -> IResult<&str, TitlePage> {
    let _result = many1(parse_titlepage_field)(input);
    let (remainder, title_page_elements) = many1(parse_titlepage_field)(input)?;

    let e = TitlePage {
        fields: {
            title_page_elements
                .iter()
                .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
                .collect()
        },
    };
    let (remainder, _) = consume_whitespace(remainder)?;
    Ok((remainder, e))
}

pub fn parse_fountain(input: &str) -> IResult<&str, FarceDocument> {
    let (remainder, (title_page, elements)) = pair(opt(parse_title_page), parse_elements)(input)?;
    Ok((
        remainder,
        FarceDocument {
            title_page: title_page,
            elements: elements,
        },
    ))
}

#[cfg(test)]
mod tests;
