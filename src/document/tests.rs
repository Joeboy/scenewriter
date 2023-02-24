#[cfg(test)]
mod tests {
    use crate::parser::parse_fountain;

    #[test]
    fn test_dialogue_as_html() {
        let input = "FRED (ABC) (EFG)\nHere's some **bold**, *italicized*, ***bold-italicized***  and _underlined_ dialogue\n";
        let (_, document) = parse_fountain(input).unwrap();
        assert_eq!(document.elements.len(), 1);
        let html = document.elements[0].as_html();
        assert_eq!(html, "<div class=\"element-dialogue\">\n<p>FRED  (ABC) (EFG)</p>\n<p>Here's some <b>bold</b>, <i>italicized</i>, <b>bold-italicized</b>  and <u>underlined</u> dialogue</p>\n</div>\n\n")
    }
}
