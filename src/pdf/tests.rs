#[cfg(test)]
mod tests {
    use crate::{document::{FarceDocument, FarceElement}, pdf::create_pdf};

    #[test]
    fn test_create_pdf() {
        let mut v = Vec::new();
        v.push(
            FarceElement::FAction("Dave is *really* **pissed**".to_string())
        );
        let fdoc = FarceDocument {
            title_page: None,
            elements: v
        };
        let _pdf_doc = create_pdf(fdoc, crate::pdf::PaperSize::A4);
    }
}
