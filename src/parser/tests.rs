//use crate::document::*;
use crate::parser::*;

use nom::IResult;

const TITLE_PAGE: &str = "Title: Big Fish
Credit: written by
Author: John August
Source: based on the novel by Daniel Wallace
Notes:	
	FINAL PRODUCTION DRAFT
	includes post-production dialogue 
	and omitted scenes\n\n";

const ELEMENTS: &str = "FRED
_Hey_ there Toby

TOBY
Hello! It's **nice** to see you!

FRED
Wish *I* could say the same

=====

INT. Somewhere in Europe

";

const ACTION: &str = "Fred and Toby,\nsitting in a tree\n\n\n";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_element() {
        let result: IResult<&str, FarceElement> =
            parse_element("FRED\nHello\nSailor\n\nextra junk here\n");

        let (remainder, element) = result.unwrap();
        assert_eq!(remainder, "extra junk here\n");

        if let FarceElement::FDialogue(d) = element {
            assert_eq!(d.character_name, "FRED");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_match_simple_title_page() {
        let result = parse_title_page("Title: Small Whale\nAuthor: Fred July\n\n");
        let (remainder, title_page) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(title_page.fields["Title"], "Small Whale");
        assert_eq!(title_page.fields["Author"], "Fred July");
    }

    #[test]
    fn test_match_title_page() {
        let result = parse_title_page(TITLE_PAGE);
        let (remainder, title_page) = result.unwrap();
        assert_eq!(remainder, "");
        assert_eq!(title_page.fields["Title"], "Big Fish");
        assert_eq!(title_page.fields["Author"], "John August");
        assert_eq!(title_page.fields["Title"], "Big Fish");
        assert_eq!(title_page.fields["Author"], "John August");
        assert_eq!(title_page.fields["Credit"], "written by");
        assert_eq!(
            title_page.fields["Source"],
            "based on the novel by Daniel Wallace"
        );
        assert_eq!(
            title_page.fields["Notes"],
            "FINAL PRODUCTION DRAFT\nincludes post-production dialogue \nand omitted scenes"
        );
    }

    #[test]
    fn test_parse_multiline_field() {
        let result = parse_multiline_titlepage_field("Notes:\n\tnote1\n\tnote2\n");
        let (_, (key, value)) = result.unwrap();
        assert_eq!(key, "Notes");
        assert_eq!(value, "note1\nnote2".to_string());
    }

    #[test]
    fn test_parse_action() {
        let result = parse_element(ACTION);
        let (remainder, element) = result.unwrap();
        assert_eq!(remainder, "");
        if let FarceElement::FAction(action) = element {
            assert_eq!(action, "Fred and Toby,\nsitting in a tree");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse_elements() {
        let result = parse_elements(ELEMENTS);
        let (remainder, elements) = result.unwrap();
        assert_eq!(elements.len(), 5);
        assert_eq!(remainder, "");
    }

    #[test]
    fn test_parse_document() {
        let input = format!("{}\n\n{}", TITLE_PAGE, ELEMENTS);
        let result = parse_fountain(input.as_str());
        let (_, document) = result.unwrap();
        let title_page = document.title_page.unwrap();
        assert_eq!(title_page.fields["Title"], "Big Fish");
        assert_eq!(title_page.fields["Author"], "John August");
        assert_eq!(document.elements.len(), 5);
    }
    #[test]
    fn test_parse_document_without_title_page() {
        let result = parse_fountain(ELEMENTS);
        let (_, document) = result.unwrap();
        let title_page = document.title_page;
        assert!(title_page.is_none());
        assert_eq!(document.elements.len(), 5);
    }

    #[test]
    fn test_parse_dialogue_multiple_extensions() {
        let (remainder, element) =
            parse_dialogue("FRED (ABC) (EFG)\nHere's some dialogue with extensions\n").unwrap();
        assert_eq!(remainder, "");
        match element {
            FarceElement::FDialogue(dialogue) => {
                assert_eq!(dialogue.character_extensions.len(), 2);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_dialogue_eof() {
        let (remainder, element) =
            parse_dialogue("FRED (ABC) (EFG)\nHere's some dialogue that ends without a newline")
                .unwrap();
        assert_eq!(remainder, "");
        match element {
            FarceElement::FDialogue(dialogue) => {
                assert_eq!(dialogue.character_extensions.len(), 2);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_action_eof() {
        let (remainder, element) = parse_element("It's an action! With no newline!").unwrap();
        assert_eq!(remainder, "");
        match element {
            FarceElement::FAction(action) => {
                assert_eq!(action, "It's an action! With no newline!");
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_dialogue_as_html() {
        let (remainder, dialogue) =
            parse_dialogue("FRED (ABC) (EFG)\nHere's some **bold**, *italicized*, ***bold-italicized***  and _underlined_ dialogue\n").unwrap();
        assert_eq!(remainder, "");
        let html = dialogue.as_html();
        assert_eq!(html, "<div class=\"element-dialogue\">\n<p>FRED  (ABC) (EFG)</p>\n<p>Here's some <b>bold</b>, <i>italicized</i>, <b>bold-italicized</b>  and <u>underlined</u> dialogue</p>\n</div>\n\n")
    }
}
