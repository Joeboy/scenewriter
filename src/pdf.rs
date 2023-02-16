use crate::parsing::FuntDocument;
use crate::parsing::FuntElement;
use genpdf::elements::Paragraph;
use genpdf::Alignment;
use genpdf::Element;
use genpdf::{elements, fonts, style};
use std::env;
use std::process::exit;

const DEFAULT_FONT_NAME: &'static str = "LiberationMono";

fn inches(inches: f32) -> f32 {
    // return mm
    inches * 25.4
}

pub fn create_pdf(fountain_doc: FuntDocument) -> Result<(), String> {
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
        .join("liberation");
    let default_font = fonts::from_files(font_dir, DEFAULT_FONT_NAME, None)
        .expect("Failed to load the default font family");

    let mut doc = genpdf::Document::new(default_font);
    doc.set_title(fountain_doc.get_title());
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.0);
    doc.set_font_size(12);
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins((inches(0.5), inches(1.0), inches(1.0), inches(1.5)));
    decorator.set_header(move |page| {
        let mut layout = elements::LinearLayout::vertical();
        if has_title_page {
            if page > 2 {
                layout.push(elements::Paragraph::new(format!("{}.", page - 1)).aligned(Alignment::Right));
                layout.push(elements::Break::new(3));  // Guestimate of 1" top margin
            } else {
                layout.push(elements::Break::new(4));
            }
        } else {
            if page > 1 {
                layout.push(elements::Paragraph::new(format!("{}.", page)).aligned(Alignment::Right));
                layout.push(elements::Break::new(3));  // Guestimate of 1" top margin
            } else {
                layout.push(elements::Break::new(4));
            }
        }
        layout.styled(style::Style::new().with_font_size(10))
    });
    doc.set_page_decorator(decorator);

    if has_title_page {
        doc.push(elements::Break::new(10));
        doc.push(
            Paragraph::new(fountain_doc.get_title())
            .aligned(Alignment::Center)
        );
        doc.push(elements::Break::new(1));
        match fountain_doc.get_author() {
            Some(author_name) => {
                doc.push(
                    Paragraph::new(fountain_doc.get_credit())
                    .aligned(Alignment::Center)
                );
                doc.push(elements::Break::new(1));
                doc.push(
                    Paragraph::new(author_name)
                    .aligned(Alignment::Center)
                );
            },
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
            FuntElement::FAction(text) => {
                doc.push(elements::Paragraph::new(text));
                doc.push(elements::Break::new(1));
            }
            FuntElement::FDialogue(dialogue) => {
                doc.push(
                    Paragraph::new(match dialogue.character_extension {
                        Some(character_extension) => {
                            format!("{} ({})", dialogue.character_name, character_extension)
                        }
                        None => dialogue.character_name,
                    })
                    .padded((0.0, 0.0, 0.0, inches(2.2))),
                );
                doc.push(Paragraph::new(dialogue.text).padded((0.0, inches(1.0), 0.0, inches(1.5))));
                doc.push(elements::Break::new(1));
            }
            FuntElement::FSceneHeading(scene_heading) => {
                doc.push(
                    elements::Paragraph::default().styled_string(
                        format!("{}. {}", scene_heading.int_or_ext, scene_heading.text),
                        style_scene_header
                ));
                doc.push(elements::Break::new(1));
            }
            FuntElement::FPageBreak => {
                doc.push(elements::PageBreak::new());
            }
        }
    }
    doc.render_to_file("./test_working.pdf")
        .expect("Failed to write output file");
    Ok(())
}
