use crate::document::*;

const TITLE_PAGE: &str = "Title: Big Fish
Credit: written by
Author: John August
Source: based on the novel by Daniel Wallace
Notes:	
	FINAL PRODUCTION DRAFT
	includes post-production dialogue 
	and omitted scenes\n\n";

const ELEMENTS: &str = "FRED
Hey there Toby

TOBY
Hello! It's nice to see you!

FRED
Wish I could say the same

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
        assert_eq!(title_page.fields["Source"], "based on the novel by Daniel Wallace");
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
        let result = parse_action(ACTION);
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
        dbg!(elements);
        assert_eq!(remainder, "");
    }

    #[test]
    fn test_parse_document() {
        let input = format!("{}\n\n{}", TITLE_PAGE, ELEMENTS);
        let result = parse_fountain(input.as_str());
        let (remainder, document) = result.unwrap();
        let title_page = document.title_page.unwrap();
        assert_eq!(title_page.fields["Title"], "Big Fish");
        assert_eq!(title_page.fields["Author"], "John August");
        assert_eq!(remainder, "");
        assert_eq!(document.elements.len(), 5);
    }
    #[test]
    fn test_parse_document_without_title_page() {
        let result = parse_fountain(ELEMENTS);
        let (remainder, document) = result.unwrap();
        let title_page = document.title_page;
        assert!(title_page.is_none());
        assert_eq!(remainder, "");
        assert_eq!(document.elements.len(), 5);
    }
}
