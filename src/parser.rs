use crate::document::*;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while, take_while1, take_while_m_n},
    character::complete::{
        alphanumeric1, char, line_ending, multispace0, not_line_ending, space0, space1,
    },
    combinator::{eof, map, opt},
    multi::{many0, many1},
    sequence::{delimited, pair, separated_pair, terminated},
    IResult,
};

fn one_or_more_non_newline_chars(input: &str) -> IResult<&str, &str> {
    let (i, line) = take_while1(|c: char| c != '\r' && c != '\n')(input)?;
    Ok((i, line))
}

fn eol_or_eof(input: &str) -> IResult<&str, &str> {
    alt((line_ending, eof))(input)
}

fn nonempty_line(input: &str) -> IResult<&str, &str> {
    let (i, line) = one_or_more_non_newline_chars(input)?;
    eol_or_eof(i)?;
    if line.trim() == "" {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Space
        )));
    }
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
            delimited(multispace0, not_line_ending, eol_or_eof),
        ),
        |s: (&str, &str)| {
            FarceElement::FSceneHeading(SceneHeading {
                int_or_ext: s.0.to_string(),
                text: s.1.to_string(),
            })
        },
    )(input)
}

fn is_character_name_char(c: char) -> bool {
    // For now let's say speaker names can only have caps and spaces
    c.is_ascii_uppercase() || c == ' ' || c.is_ascii_digit()
}

fn parse_character_extension(input: &str) -> IResult<&str, &str> {
    let (i, extension) = delimited(tag("("), is_not(")"), tag(")"))(input)?;
    let (i, _whitespace) = space0(i)?;
    Ok((i, extension))
}

fn parse_character_name(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (i, name) = take_while(|c: char| is_character_name_char(c))(input)?;
    let (_, _) = alphanumeric1(name)?;
    let (i, _whitespace) = space0(i)?;
    let (i, extensions) = many0(parse_character_extension)(i)?;
    Ok((i, (name.trim(), extensions)))
}

fn parse_dialogue(input: &str) -> IResult<&str, FarceElement> {
    let (i, (character_name, extensions)) = terminated(parse_character_name, line_ending)(input)?;
    let (remainder, lines) = many1(terminated(nonempty_line, opt(line_ending)))(i)?;
    let e = FarceElement::FDialogue(Dialogue {
        character_name: String::from(character_name),
        character_extensions: extensions.iter().map(|s| s.to_string()).collect(),
        text: String::from(lines.join(" ")),
    });
    Ok((remainder, e))
}

fn parse_action(input: &str) -> IResult<&str, FarceElement> {
    let (remainder, lines) = many1(terminated(nonempty_line, opt(line_ending)))(input)?;

    Ok((
        remainder,
        FarceElement::FAction(Action {
            text: lines.join("\n"),
            is_centered: false,
        }),
    ))
}

fn parse_centered_action(input: &str) -> IResult<&str, FarceElement> {
    // For now let's assume this has to be one line only
    let (remainder, line) = terminated(
        delimited(
            tag(">"),
            take_while1(|c| c != '\r' && c != '\n' && c != '<'), // Not sure why not_newline doesn't work here?
            tag("<"),
        ),
        eol_or_eof,
    )(input)?;
    Ok((
        remainder,
        FarceElement::FAction(Action {
            text: line.trim().to_string(),
            is_centered: true,
        }),
    ))
}

fn parse_page_break(input: &str) -> IResult<&str, FarceElement> {
    let (remainder, _) =
        terminated(take_while_m_n(3, 1e23 as usize, |s| s == '='), eol_or_eof)(input)?;
    Ok((remainder, FarceElement::FPageBreak))
}

pub fn parse_element(input: &str) -> IResult<&str, FarceElement> {
    let (remainder, element) = alt((
        parse_scene_heading,
        parse_dialogue,
        parse_page_break,
        parse_centered_action,
        parse_action,
    ))(input)?;
    let (remainder, _) = consume_whitespace(remainder)?;
    Ok((remainder, element))
}

pub fn parse_elements(input: &str) -> IResult<&str, Vec<FarceElement>> {
    many0(parse_element)(input)
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
    let result = pair(opt(parse_title_page), parse_elements)(&input);
    match result {
        Ok((remainder, (title_page, elements))) => Ok((
            remainder,
            FarceDocument {
                title_page: title_page,
                elements: elements,
            },
        )),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests;
