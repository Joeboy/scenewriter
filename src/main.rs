use crate::parsing::FarceElement;
use crate::pdf::create_pdf;
use std::env;
use std::fs;
use std::process::exit;
mod parsing;
mod pdf;


fn main() {
    let args: Vec<String> = env::args().collect();
    let _input;
    let input = match args.len() {
        //1 => INPUT,
        2 => {
            match fs::read_to_string(&args[1]) {
                Ok(s) => {
                    _input = s;
                }
                Err(err) => {
                    eprintln!("Couldn't read input file {} ({})", &args[1], err);
                    exit(1);
                }
            }
            &_input
        }
        _ => {
            eprintln!("Couldn't parse command line arguments");
            exit(1);
            ""
        }
    };

    let result = parsing::parse_document(input);
    match result {
        Ok((remaining_input, document)) => {
            if let Some(ref title_page) = document.title_page {
                println!("===== TITLE PAGE ====");
                for field in &title_page.fields {
                    println!("{} - {}", field.0, field.1);
                }
                println!("=====================");
            }
            for element in &document.elements {
                match element {
                    FarceElement::FDialogue(dialogue) => {
                        match &dialogue.character_extension {
                            Some(extension) => {
                                println!("Dialogue: {} (({}))", dialogue.character_name, extension)
                            }
                            None => {
                                println!("Dialogue: {}: {}", dialogue.character_name, dialogue.text)
                            }
                        };
                    }
                    FarceElement::FSceneHeading(text) => {
                        println!("Scene heading: {}", text);
                    }
                    FarceElement::FAction(text) => {
                        println!("Action: {}", text);
                    }
                    FarceElement::FPageBreak => {
                        println!("<Page Break>");
                    }
                };
                println!();
            }
            println!(" ==== Remaining input ====\n\n {}", remaining_input);
            match create_pdf(document) {
                Ok(()) => {},
                Err(e) => {
                    eprintln!("Couldn't generate PDF ({})", e);
                    exit(1)
                }
            }
        }
        Err(error) => {
            println!("Parsing error: {:?}", error);
        }
    }
}
