#[cfg(test)]
mod tests {
    use crate::{
        document::{FarceDocument, FarceElement},
        pdf::create_pdf,
    };

    #[test]
    fn test_create_pdf() {
        let mut v = Vec::new();
        v.push(FarceElement::FAction(
            "Here's some *italic*, _underlined_, **bold** and ***Bold / italic*** text."
                .to_string(),
        ));
        let fdoc = FarceDocument {
            title_page: None,
            elements: v,
        };
        let _pdf_doc = create_pdf(fdoc, crate::pdf::PaperSize::A4).unwrap();
        _pdf_doc.render_to_file("t.pdf").unwrap();
        // Should probably actually test the pdf somehow.
        // NB As of now, underlined text is not supported
    }
}
