use crate::constants;
use crate::document::{FarceDocument, FarceElement};
use genpdf;
use genpdf::{elements, elements::Paragraph, fonts, style, Alignment, Element};
use std::env;
use std::fmt;
use std::process::exit;

fn inches(inches: f32) -> f32 {
    // return mm
    inches * 25.4
}
#[derive(Copy, Clone, Debug)]
pub enum PaperSize {
    A4,
    Letter,
}
impl PaperSize {
    fn get_genpdf_paper_size(&self) -> genpdf::PaperSize {
        match self {
            PaperSize::A4 => genpdf::PaperSize::A4,
            PaperSize::Letter => genpdf::PaperSize::Letter,
        }
    }
}
impl fmt::Display for PaperSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaperSize::A4 => {
                write!(f, "A4")
            }
            PaperSize::Letter => {
                write!(f, "US Letter")
            }
        }
    }
}

// struct PageDimensions(f32, f32);

// impl PaperSize {
//     fn get_dimensions(&self) -> PageDimensions {
//         match self {
//             PaperSize::A4 => PageDimensions(297.0, 210.0),
//             PaperSize::Letter => PageDimensions(279.0, 196.0),
//         }
//     }
// }

pub fn create_pdf(
    fountain_doc: FarceDocument,
    paper_size: PaperSize,
) -> Result<genpdf::Document, String> {
    let title = {
        match fountain_doc.get_title() {
            Some(title) => title,
            None => constants::DEFAULT_TITLE,
        }
    };
    let has_title_page = fountain_doc.has_title_page();
    let exe_path = match env::current_exe() {
        Ok(exe_path) => exe_path,
        Err(e) => {
            eprintln!("Couldn't get executable directory ({})", e);
            exit(1)
        }
    };
    let font_dir = exe_path
        .parent()
        .unwrap()
        .join("fonts")
        .join("truetype")
        .join("Courier Prime");
    let default_font = fonts::from_files(font_dir, constants::DEFAULT_FONT_NAME, None)
        .expect("Failed to load the default font family");

    let mut doc = genpdf::Document::new(default_font);
    doc.set_paper_size(paper_size.get_genpdf_paper_size());
    doc.set_title(title);
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.0);
    doc.set_font_size(12);
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins((inches(0.5), inches(0.8), inches(1.0), inches(1.5)));
    decorator.set_header(move |page| {
        let mut layout = elements::LinearLayout::vertical();
        if has_title_page {
            if page > 2 {
                layout.push(
                    elements::Paragraph::new(format!("{}.", page - 1)).aligned(Alignment::Right),
                );
                layout.push(elements::Break::new(3)); // Guestimate of 1" top margin
            } else {
                layout.push(elements::Break::new(4));
            }
        } else {
            if page > 1 {
                layout
                    .push(elements::Paragraph::new(format!("{}.", page)).aligned(Alignment::Right));
                layout.push(elements::Break::new(3)); // Guestimate of 1" top margin
            } else {
                layout.push(elements::Break::new(4));
            }
        }
        layout.styled(style::Style::new().with_font_size(10))
    });
    doc.set_page_decorator(decorator);

    if has_title_page {
        doc.push(elements::Break::new(10));
        doc.push(Paragraph::new(title).aligned(Alignment::Center));
        doc.push(elements::Break::new(1));
        match fountain_doc.get_titlepage_field("Author") {
            Some(author_name) => {
                doc.push(
                    Paragraph::new({
                        match fountain_doc.get_titlepage_field("Credit") {
                            Some(credit) => credit,
                            None => constants::DEFAULT_CREDIT,
                        }
                    })
                    .aligned(Alignment::Center),
                );
                doc.push(elements::Break::new(1));
                doc.push(Paragraph::new(author_name).aligned(Alignment::Center));
            }
            None => {}
        }
        doc.push(elements::PageBreak::new());
    }
    #[cfg(feature = "hyphenation")]
    {
        use hyphenation::Load;

        doc.set_hyphenator(
            hyphenation::Standard::from_embedded(hyphenation::Language::EnglishUS)
                .expect("Failed to load hyphenation data"),
        );
    }

    let style_scene_header = style::Style::from(style::Effect::Bold);

    // doc.push(
    //     elements::Paragraph::default()
    //         .string("You can also ")
    //         .styled_string("combine ", red)
    //         .styled_string("multiple ", style::Style::from(blue).italic())
    //         .styled_string("formats", code)
    //         .string(" in one paragraph.")
    //         .styled(style::Style::new().with_font_size(16)),
    // );
    for element in fountain_doc.elements {
        match element {
            FarceElement::FAction(text) => {
                doc.push(elements::Paragraph::new(text));
                doc.push(elements::Break::new(1));
            }
            FarceElement::FDialogue(dialogue) => {
                doc.push(Paragraph::new(dialogue.character_line_as_text()).padded((
                    0.0,
                    0.0,
                    0.0,
                    inches(1.9),
                )));
                doc.push(Paragraph::new(dialogue.text).padded((
                    0.0,
                    inches(1.3),
                    0.0,
                    inches(0.875),
                )));
                doc.push(elements::Break::new(1));
            }
            FarceElement::FSceneHeading(scene_heading) => {
                doc.push(elements::Paragraph::default().styled_string(
                    format!("{}. {}", scene_heading.int_or_ext, scene_heading.text),
                    style_scene_header,
                ));
                doc.push(elements::Break::new(1));
            }
            FarceElement::FPageBreak => {
                doc.push(elements::PageBreak::new());
            }
        }
    }
    Ok(doc)
}
