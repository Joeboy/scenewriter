use crate::constants;
use crate::document::{FarceDocument, FarceElement};
use crate::inline_parser::{parse_inline, Expression};
use allsorts::{
    binary::read::ReadScope,
    font::read_cmap_subtable,
    subset::subset,
    tables::{cmap::Cmap, FontTableProvider},
    tag,
};
use genpdf;
use genpdf::elements::{Alignment, Paragraph};
use genpdf::fonts::FontFamily;
use genpdf::{elements, fonts, style, Element};
use include_dir::{include_dir, Dir};
use std::fmt;

static FONTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/fonts/truetype/Courier Prime");
const SKIP_FONT_SUBSETTING: bool = false;

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

struct TextState {
    bold: bool,
    italic: bool,
    underline: bool,
}

impl TextState {
    fn get_genpdf_style(&self) -> style::Style {
        let mut style = style::Style::default();
        if self.bold {
            style.merge(style::Effect::Bold);
        };
        if self.italic {
            style.merge(style::Effect::Italic);
        };
        if self.underline {
            style.merge(style::Effect::Underlined);
        };
        style
    }
}

fn render_text_elements(
    p: &mut Paragraph,
    expressions: &Vec<Expression>,
    text_state: &mut TextState,
) {
    for e in expressions {
        match e {
            Expression::Text(t) => {
                p.push(style::StyledString {
                    s: t.to_string(),
                    style: text_state.get_genpdf_style(),
                });
            }
            Expression::Bold(v) => {
                text_state.bold = true;
                render_text_elements(p, &v, text_state);
                text_state.bold = false;
            }
            Expression::Italic(v) => {
                text_state.italic = true;
                render_text_elements(p, &v, text_state);
                text_state.italic = false;
            }
            Expression::BoldItalic(v) => {
                text_state.bold = true;
                text_state.italic = true;
                render_text_elements(p, &v, text_state);
                text_state.bold = false;
                text_state.italic = false;
            }
            Expression::Underline(v) => {
                // Not actually supported (yet?)
                text_state.underline = true;
                render_text_elements(p, &v, text_state);
                text_state.underline = false;
            }
        }
    }
}

fn render_inline_formatting(text: &str, is_centered: bool) -> Paragraph {
    let mut p: Paragraph = Paragraph::default();
    let mut_ref = &mut p;
    match parse_inline(&text) {
        Ok((_remainder, expressions)) => {
            let mut text_state = TextState {
                bold: false,
                italic: false,
                underline: false,
            };
            render_text_elements(mut_ref, &expressions, &mut text_state);
            match is_centered {
                true => p.aligned(Alignment::Center),
                false => p,
            }
        }
        Err(e) => {
            p.push(format!("{}", e));
            p
        }
    }
}

fn get_fontdata(font_filename: &str, subset_chars: &Vec<char>) -> fonts::FontData {
    let f = FONTS_DIR
        .get_file(&font_filename)
        .expect("Couldn't open font file");
    let data = f.contents().to_vec();
    if SKIP_FONT_SUBSETTING {
        fonts::FontData::new(data, None).unwrap()
    } else {
        // To avoid bloating the PDF, we only embed the glyphs we need
        let font_file = ReadScope::new(&data)
            .read::<allsorts::font_data::FontData<'_>>()
            .unwrap();
        let provider = font_file.table_provider(0).unwrap();

        let cmap_data = provider.read_table_data(tag::CMAP).unwrap();
        let cmap = ReadScope::new(&cmap_data).read::<Cmap<'_>>().unwrap();
        let (_, cmap_subtable) = read_cmap_subtable(&cmap).unwrap().unwrap();
        let mut glyph_ids = vec![
            0 as u16,
            // Force unicode encoding:
            cmap_subtable.map_glyph('€' as u32).unwrap().unwrap(),
        ];
        glyph_ids.extend(
            subset_chars
                .iter()
                // TODO: pretty sure this'll fail disgracefully with unsupported chars
                .map(|c| cmap_subtable.map_glyph(*c as u32).unwrap().unwrap()),
        );
        glyph_ids.sort();
        glyph_ids.dedup();

        let new_font = subset(&provider, &glyph_ids).unwrap();
        fonts::FontData::new(new_font, None).unwrap()
    }
}

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

    let all_chars = fountain_doc.get_all_chars();
    let default_font = FontFamily {
        regular: get_fontdata("Courier Prime Regular.ttf", &all_chars),
        italic: get_fontdata("Courier Prime Italic.ttf", &all_chars),
        bold: get_fontdata("Courier Prime Bold.ttf", &all_chars),
        bold_italic: get_fontdata("Courier Prime BoldItalic.ttf", &all_chars),
    };

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

    for element in fountain_doc.elements {
        match element {
            FarceElement::FAction(action) => {
                doc.push(render_inline_formatting(&action.text, action.is_centered));
                doc.push(elements::Break::new(1));
            }
            FarceElement::FDialogue(dialogue) => {
                doc.push(Paragraph::new(dialogue.character_line_as_text()).padded((
                    0.0,
                    0.0,
                    0.0,
                    inches(1.9),
                )));
                doc.push(render_inline_formatting(&dialogue.text, false).padded((
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
                    style::Style::from(style::Effect::Bold),
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

#[cfg(test)]
mod tests;
