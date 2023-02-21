use crate::parsing::FarceElement;
use crate::pdf::create_pdf;
use std::env;
use std::fmt;
use std::fs;
use std::path::Path;
use std::process::exit;
mod parsing;
mod pdf;

enum OutputMode {
    Html,
    Pdf,
}

impl fmt::Display for OutputMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputMode::Html => {
                write!(f, "Html mode")
            }
            OutputMode::Pdf => {
                write!(f, "Pdf mode")
            }
        }
    }
}

fn print_usage() {
    println!("Usage: farce [--output [filename]] [--pdf|html] input.fountain");
    exit(1)
}

fn main() {
    let mut args = env::args().skip(1);
    let mut input_filename: Option<&str> = None;
    let mut output_filename: Option<String> = None;
    let mut positional_args = Vec::new();
    let mut output_mode: OutputMode = OutputMode::Pdf;

    while let Some(arg) = args.next() {
        match &arg[..] {
            "--pdf" => {
                output_mode = OutputMode::Pdf;
            }
            "--html" => {
                output_mode = OutputMode::Html;
            }
            "--output" | "-o" => {
                output_filename = args.next();
                if output_filename.is_none() {
                    eprintln!("No value specified for parameter {}", &arg);
                    print_usage()
                }
            }
            "--usage" => print_usage(),
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
            input_filename = Some(&positional_args[0]);
        }
        _ => {
            eprintln!("Couldn't parse commandline args");
            print_usage()
        }
    }

    if output_filename.is_none() {
        let input_path = Path::new(input_filename.unwrap());
        let file_stem = input_path.file_stem().unwrap().to_str().unwrap();
        let suffix = match output_mode {
            OutputMode::Html => "html",
            OutputMode::Pdf => "pdf",
        };
        output_filename = Some(format!("{}.{}", file_stem, suffix));
    }

    println!("Input file: {}", &input_filename.as_ref().unwrap());
    println!("Output file: {}", &output_filename.as_ref().unwrap());
    println!("Output mode: {}", output_mode);

    let input = match fs::read_to_string(input_filename.unwrap()) {
        Ok(s) => s,
        Err(err) => {
            eprintln!(
                "Couldn't read input file {} ({})",
                input_filename.unwrap(),
                err
            );
            exit(1)
        }
    };

    match parsing::parse_document(&input) {
        Ok((remaining_input, document)) => {
            println!("==== Unhandled input ====\n\n {}", remaining_input);

            match output_mode {
                OutputMode::Pdf => match create_pdf(document, &output_filename.unwrap()) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("Couldn't generate PDF ({})", e);
                        exit(1)
                    }
                },
                OutputMode::Html => {
                    eprintln!("Html output not supported just yet, sorry");
                    exit(1)
                }
            }
        }
        Err(error) => {
            println!("Parsing error: {:?}", error);
        }
    }
}
