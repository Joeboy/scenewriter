use crate::constants::{DEFAULT_CREDIT, DEFAULT_TITLE};
use crate::document::{FarceDocument, TitlePage};
use std::fs::File;
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

fn write_title_page(title_page: &TitlePage, mut f: &File) {
    f.write(b"<div id=\"title-page\">").unwrap();

    f.write(b"<div id=\"title-page-credits\">").unwrap();
    match title_page.fields.get("Title") {
        Some(title) => {
            f.write(format!("<p>{}</p>", title).as_bytes()).unwrap();
        }
        None => {
            f.write(DEFAULT_TITLE.as_bytes()).unwrap();
        }
    }

    match title_page.fields.get("Author") {
        Some(author) => {
            let default_credit = DEFAULT_CREDIT.to_string();
            let credit = title_page.fields.get("Credit").unwrap_or(&default_credit);
            f.write(format!("<p>{}</p>", credit).as_bytes()).unwrap();
            f.write(format!("<p>{}</p>", author).as_bytes()).unwrap();
        }
        None => (),
    }
    f.write(b"</div>").unwrap();

    f.write(b"</div>").unwrap();
}

pub fn write_html(document: FarceDocument, mut f: File) -> Result<(), String> {
    f.write(HTML_HEADER).unwrap();
    if let Some(ref title_page) = document.title_page {
        write_title_page(title_page, &f);
    }
    for element in &document.elements {
        f.write(element.as_html().as_bytes()).unwrap();
    }
    f.write(HTML_FOOTER).unwrap();
    Ok(())
}
