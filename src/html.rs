use crate::constants::{DEFAULT_CREDIT, DEFAULT_TITLE};
use crate::document::{FarceDocument, TitlePage};
use std::io::Write;

const HTML_HEADER: &[u8] = b"<html><head><style type=\"text/css\">
body { font-family: Courier; width: 800px; margin-left: 200px;}
p {margin: 0px;}
p {padding: 0px;}
.element-dialogue {padding-left: 100px; padding-right: 200px;}
div::after { content: \"\\00a0\";}
div.element-pagebreak {break-after:page; padding-bottom: 250px; }
div#title-page-credits {text-align: center; margin: 200px auto 200px auto;}
</style></head>\n\n<body>";

const HTML_FOOTER: &[u8] = b"</body></html>";

fn write_title_page(title_page: &TitlePage, w: &mut impl Write) {
    w.write(b"<div id=\"title-page\">").unwrap();

    w.write(b"<div id=\"title-page-credits\">").unwrap();
    match title_page.fields.get("Title") {
        Some(title) => {
            w.write(format!("<p>{}</p>", title).as_bytes()).unwrap();
        }
        None => {
            w.write(DEFAULT_TITLE.as_bytes()).unwrap();
        }
    }

    match title_page.fields.get("Author") {
        Some(author) => {
            let default_credit = DEFAULT_CREDIT.to_string();
            let credit = title_page.fields.get("Credit").unwrap_or(&default_credit);
            w.write(format!("<p>{}</p>", credit).as_bytes()).unwrap();
            w.write(format!("<p>{}</p>", author).as_bytes()).unwrap();
        }
        None => (),
    }
    w.write(b"</div>").unwrap();

    w.write(b"</div>").unwrap();
}

pub fn write_html(
    document: FarceDocument,
    mut w: impl Write,
    include_header_and_footer: bool,
) -> Result<(), String> {
    if include_header_and_footer {
        w.write(HTML_HEADER).unwrap();
    }
    if let Some(ref title_page) = document.title_page {
        write_title_page(title_page, &mut w);
    }
    for element in &document.elements {
        w.write(element.as_html().as_bytes()).unwrap();
    }
    if include_header_and_footer {
        w.write(HTML_FOOTER).unwrap();
    }
    Ok(())
}
