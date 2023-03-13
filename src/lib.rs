pub mod constants;
pub mod document;
pub mod html;
pub mod inline_parser;
pub mod parser;
pub mod pdf;
pub mod utils;

use std::io;

struct MyWriter {
    buffer: Vec<u8>,
}

impl MyWriter {
    // Maybe it's possible to do this with a Cursor object?
    // Couldn't make it work so here is this.
    fn new() -> MyWriter {
        MyWriter { buffer: Vec::new() }
    }

    fn into_inner(self) -> Vec<u8> {
        self.buffer
    }
}

impl io::Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}

pub fn fountain_to_pdf(input: &str, paper_size: pdf::PaperSize) -> Vec<u8> {
    let (_remainder, fdoc) = parser::parse_fountain(input).expect("Could not parse fountain doc");
    let genpdf_doc = pdf::create_pdf(fdoc, paper_size).expect("Could not create pdf");
    let mut writer = MyWriter::new();
    genpdf_doc
        .render(&mut writer)
        .expect("Failed to render pdf");
    writer.into_inner()
}

pub fn fountain_to_html(input: &str) -> String {
    let (_remainder, fdoc) = parser::parse_fountain(input).expect("Could not parse fountain doc");
    let mut writer = MyWriter::new();
    html::write_html(fdoc, &mut writer).expect("Failed to write html");
    let bytes = writer.into_inner();
    String::from_utf8(bytes).expect("Could not decode html as utf8")
}
