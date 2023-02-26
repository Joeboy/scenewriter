// Markdownesque formatting for inline fountain text
// Per https://fountain.io/syntax#section-emphasis
//
// *italics*
// **bold**
// ***bold italics***
// _underline_

// Somewhat borrowed from https://imfeld.dev/writing/parsing_with_nom
// Thanks Daniel Imfield

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::{map, map_parser},
    error::context,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Text(String),
    Italic(Vec<Expression>),
    Bold(Vec<Expression>),
    BoldItalic(Vec<Expression>),
    Underline(Vec<Expression>),
}

impl Expression {
    pub fn as_html<'a>(&self) -> String {
        let s: String;
        match self {
            Expression::Text(t) => t.to_string(),
            Expression::Italic(expressions) => {
                s = expressions
                    .iter()
                    .map(|e| e.as_html())
                    .collect::<Vec<String>>()
                    .join("");
                format!("<i>{}</i>", s)
            }
            Expression::Bold(expressions) => {
                s = expressions
                    .iter()
                    .map(|e| e.as_html())
                    .collect::<Vec<String>>()
                    .join("");
                format!("<b>{}</b>", s)
            }
            Expression::BoldItalic(expressions) => {
                s = expressions
                    .iter()
                    .map(|e| e.as_html())
                    .collect::<Vec<String>>()
                    .join("");
                format!("<b>{}</b>", s)
            }
            Expression::Underline(expressions) => {
                s = expressions
                    .iter()
                    .map(|e| e.as_html())
                    .collect::<Vec<String>>()
                    .join("");
                format!("<u>{}</u>", s)
            }
        }
    }
}

fn fenced<'a>(start: &'a str, end: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    map(tuple((tag(start), take_until(end), tag(end))), |x| x.1)
}

fn style<'a>(boundary: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Expression>> {
    map_parser(fenced(boundary, boundary), parse_inline)
}

fn bold(input: &str) -> IResult<&str, Vec<Expression>> {
    style("**")(input)
}

fn italic(input: &str) -> IResult<&str, Vec<Expression>> {
    style("*")(input)
}

fn bold_italic(input: &str) -> IResult<&str, Vec<Expression>> {
    style("***")(input)
}

fn underline(input: &str) -> IResult<&str, Vec<Expression>> {
    style("_")(input)
}

fn directive(input: &str) -> IResult<&str, Expression> {
    alt((
        // NB order is important here
        map(bold_italic, Expression::BoldItalic),
        map(context("bold", bold), Expression::Bold),
        map(italic, Expression::Italic),
        map(underline, Expression::Underline),
    ))(input)
}

/// Parse a line of text, counting anything that doesn't match a directive as plain text.
pub fn parse_inline(input: &str) -> IResult<&str, Vec<Expression>> {
    let mut output = Vec::with_capacity(4);

    let mut current_input = input;

    while !current_input.is_empty() {
        let mut found_directive = false;
        for (current_index, _) in current_input.char_indices() {
            // println!("{} {}", current_index, current_input);
            match directive(&current_input[current_index..]) {
                Ok((remaining, parsed)) => {
                    // println!("Matched {:?} remaining {}", parsed, remaining);
                    let leading_text = &current_input[0..current_index];
                    if !leading_text.is_empty() {
                        output.push(Expression::Text(leading_text.to_string()));
                    }
                    output.push(parsed);

                    current_input = remaining;
                    found_directive = true;
                    break;
                }
                Err(nom::Err::Error(_)) => {
                    // None of the parsers matched at the current position, so this character is just part of the text.
                    // The iterator will go to the next character so there's nothing to do here.
                }
                Err(e) => {
                    // On any other error, just return the error.
                    return Err(e);
                }
            }
        }

        if !found_directive {
            output.push(Expression::Text(current_input.to_string()));
            break;
        }
    }

    Ok(("", output))
}

#[cfg(test)]
mod tests {
    use crate::inline_parser::parse_inline;
    #[test]
    fn test_parse_inline() {
        let (_remainder, expressions) =
            parse_inline("Dave is *actually* **really** ***pissed*** _now_.").unwrap();
        assert_eq!(expressions.len(), 9);
    }
}
