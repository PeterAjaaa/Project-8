use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use camino::Utf8Path;
use clap::Parser;

/// The struct representing the arguments passed to the compiler
#[derive(Parser, Debug)]
struct Args {
    /// The filename or filepath representing the brainf*ck file to be processed by the compiler
    file: Option<String>,
}

fn main() {
    /// Memory cells length to be used as the data tape. Official implementation used 30k cells.
    const MEMORY_CELLS_LENGTH: usize = 30000;

    let mem_cells = vec![0, MEMORY_CELLS_LENGTH];
    let args = Args::parse();

    match args.file {
        Some(filename) => {
            let path = Utf8Path::new(&filename);

            match path.try_exists() {
                Ok(_) => match path.canonicalize_utf8() {
                    Ok(path) => {
                        let file = File::open(path).unwrap();
                        let reader = BufReader::new(file);

                        for line in reader.lines() {
                            match line {
                                Ok(content) => {
                                    println!("{}", content);
                                }
                                Err(e) => {
                                    eprintln!("{}", e);
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                },
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
        None => {
            let rl = rustyline::DefaultEditor::new();

            match rl {
                Ok(mut editor) => {
                    let readline = editor.readline("project-8>> ");
                    match readline {
                        Ok(line) => println!("{}", line),
                        Err(e) => eprintln!("{}", e),
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    }
}
