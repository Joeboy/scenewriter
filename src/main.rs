mod constants;
mod document;
mod html;
mod inline_parser;
mod parser;
mod pdf;
mod stats;
mod utils;

use crate::html::write_html;
use crate::pdf::create_pdf;
use crate::stats::print_stats;

use std::env;
use std::fmt;
use std::fs;
use std::path::Path;
use std::process::exit;

#[derive(Copy, Clone, Debug)]
enum OutputMode {
    Html,
    Pdf,
    Stats,
}

impl OutputMode {
    fn get_suffix(&self) -> Option<String> {
        match self {
            OutputMode::Html => Some(String::from("html")),
            OutputMode::Pdf => Some(String::from("pdf")),
            OutputMode::Stats => None,
        }
    }
}
impl fmt::Display for OutputMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputMode::Html => {
                write!(f, "Write HTML file")
            }
            OutputMode::Pdf => {
                write!(f, "Write PDF file")
            }
            OutputMode::Stats => {
                write!(f, "Show stats")
            }
        }
    }
}

fn print_usage() {
    println!();
    println!("Usage: farce [..options..] input_filename");
    println!();
    println!("Options:");
    println!();
    println!("             --pdf     Write PDF file (default)");
    println!("            --html     Write HTML file");
    println!("           --stats     Show screenplay stats");
    println!();
    println!("              --a4     A4 page size (default)");
    println!("                -a");
    println!();
    println!("          --letter     US Letter page size");
    println!("                -l");
    println!();
    println!(" --output filename     Choose output filename (default is the input");
    println!("       -o filename     filename but with .pdf or .html extension)");
    println!();
    println!("            --help     Show this help");
    println!();
    println!();
    println!(
        "      Eg: farce --a4 --pdf -o \"My Screenplay final-v23.pdf\" my_screenplay.fountain"
    );
    println!();
    println!(" or just: farce my_screenplay.fountain");
    println!();
    exit(1)
}

fn main() {
    let mut args = env::args().skip(1);
    let mut maybe_input_filename: Option<&str> = None;
    let input_filename: &str;
    let mut maybe_output_filename: Option<String> = None;
    let output_filename: Option<&str>;
    let mut positional_args = Vec::new();
    let mut requested_paper_sizes = Vec::new();
    let paper_size: pdf::PaperSize;
    let mut requested_output_modes = Vec::new();
    let output_mode: OutputMode;

    while let Some(arg) = args.next() {
        match &arg[..] {
            "--pdf" => {
                requested_output_modes.push(OutputMode::Pdf);
            }
            "--html" => {
                requested_output_modes.push(OutputMode::Html);
            }
            "--stats" => {
                requested_output_modes.push(OutputMode::Stats);
            }
            "--output" | "-o" => {
                maybe_output_filename = args.next();
                if maybe_output_filename.is_none() {
                    eprintln!("No value specified for parameter {}", &arg);
                    print_usage()
                }
            }
            "--a4" | "-a" => {
                requested_paper_sizes.push(pdf::PaperSize::A4);
            }
            "--letter" | "-l" => {
                requested_paper_sizes.push(pdf::PaperSize::Letter);
            }
            "--help" => print_usage(),
            _ => {
                if arg.starts_with('-') {
                    println!("Unrecognized argument {}", arg);
                    print_usage()
                } else {
                    positional_args.push(arg);
                }
            }
        }
    }

    match positional_args.len() {
        0 => {
            eprintln!("No input file specified");
            print_usage()
        }
        1 => {
            maybe_input_filename = Some(&positional_args[0]);
        }
        _ => {
            eprintln!("Couldn't parse commandline args");
            print_usage()
        }
    }
    input_filename = maybe_input_filename.unwrap();

    match requested_output_modes.len() {
        0 => {
            output_mode = OutputMode::Pdf;
        }
        1 => {
            output_mode = requested_output_modes[0];
        }
        _ => {
            eprintln!("Please choose only one of --pdf, --html and --stats");
            exit(1)
        }
    }

    let output_filename_string: String;
    output_filename = match output_mode {
        OutputMode::Html | OutputMode::Pdf => match maybe_output_filename {
            Some(ref of) => Some(of),
            None => {
                let input_path = Path::new(input_filename);
                let file_stem = input_path.file_stem().unwrap().to_str().unwrap();
                let suffix = output_mode.get_suffix().unwrap();
                output_filename_string = format!("{}.{}", file_stem, suffix).to_string();
                Some(&output_filename_string)
            }
        },
        OutputMode::Stats => None,
    };

    match requested_paper_sizes.len() {
        0 => {
            paper_size = pdf::PaperSize::A4;
        }
        1 => {
            paper_size = requested_paper_sizes[0];
        }
        _ => {
            eprintln!("Multiple page sizes requested");
            exit(1)
        }
    }

    println!("Input file: {}", input_filename);
    println!("Output mode: {}", output_mode);
    match output_filename {
        Some(of) => {
            println!("Output file: {}", of);
        }
        None => {}
    }
    println!("Page size: {}", paper_size);

    let input = match fs::read_to_string(input_filename) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Couldn't read input file {} ({})", input_filename, err);
            exit(1)
        }
    };

    match parser::parse_fountain(&input) {
        Ok((_remainder, document)) => match output_mode {
            OutputMode::Pdf => match create_pdf(document, paper_size) {
                Ok(genpdf_document) => {
                    genpdf_document
                        .render_to_file(output_filename.unwrap())
                        .expect("Failed to write output file");
                }
                Err(e) => {
                    eprintln!("Couldn't generate PDF ({})", e);
                    exit(1)
                }
            },
            OutputMode::Html => {
                let f = fs::File::create(output_filename.unwrap()).expect(&format!(
                    "Could not open file {} for writing",
                    output_filename.unwrap()
                ));
                write_html(document, f).unwrap();
            }
            OutputMode::Stats => {
                print_stats(&document);
            }
        },
        Err(error) => {
            println!("Parsing error: {:?}", error);
            exit(1)
        }
    }
}
