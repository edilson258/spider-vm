mod analysis;
mod ast;
mod codegeneration;
mod frontend;
mod utils;

use std::io::Write;
use std::process::exit;
use std::{env, fs};

use utils::read_file;

use crate::analysis::Analyser;
use crate::codegeneration::CodeGenerator;
use frontend::{lexer::Lexer, parser::Parser};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("[Error]: No input file provided");
        exit(1);
    }
    let file_content = match read_file(&args[1]) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!(
                "[Error]: Couldn't read file {} {}",
                args[1],
                err.to_string()
            );
            exit(1);
        }
    };
    let input = file_content.to_string().chars().collect::<Vec<char>>();
    let mut l = Lexer::new(&input);
    let mut p = Parser::new(&mut l);

    let mut ast = match p.parse() {
        Ok(ast) => ast,
        Err(err) => {
            println!("[Syntax Error]: {}", err);
            exit(1);
        }
    };

    let mut analiser = Analyser::make();
    match analiser.analyse(&mut ast) {
        Ok(()) => {}
        Err(errors) => {
            for err in errors {
                eprintln!("{}", err);
            }
            exit(1);
        }
    }

    let mut generator = CodeGenerator::make();
    let program = generator.gen(ast);

    let bin = bincode::serialize(&program).unwrap();

    let mut file = fs::File::create("out.bin").unwrap();
    file.write(&bin).unwrap();
}
