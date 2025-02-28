use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let mut exit_code = 0;
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut line_number = 1;
                let mut file_content_chars = file_contents.chars().peekable();
                while let Some(char) = file_content_chars.next() {
                    match char {
                        '(' => println!("LEFT_PAREN ( null"),
                        ')' => println!("RIGHT_PAREN ) null"),
                        '{' => println!("LEFT_BRACE {{ null"),
                        '}' => println!("RIGHT_BRACE }} null"),
                        ',' => println!("COMMA , null"),
                        '.' => println!("DOT . null"),
                        '-' => println!("MINUS - null"),
                        '+' => println!("PLUS + null"),
                        ';' => println!("SEMICOLON ; null"),
                        '*' => println!("STAR * null"),
                        '=' => match file_content_chars.peek() {
                            Some('=') => {
                                file_content_chars.next();
                                println!("EQUAL_EQUAL == null");
                            },
                            _ => println!("EQUAL = null"),
                        },
                        '!' => match file_content_chars.peek() {
                            Some('=') => {
                                file_content_chars.next();
                                println!("BANG_EQUAL != null");
                            },
                            _ => println!("BANG ! null"),
                        },
                        '<' => match file_content_chars.peek() {
                            Some('=') => {
                                file_content_chars.next();
                                println!("LESS_EQUAL <= null");
                            },
                            _ => println!("LESS < null"),
                        },
                        '>' => match file_content_chars.peek() {
                            Some('=') => {
                                file_content_chars.next();
                                println!("GREATER_EQUAL >= null");
                            },
                            _ => println!("GREATER > null"),
                        },
                        '\n' => line_number += 1,
                        _ => {
                            eprintln!("[line {}] Error: Unexpected character: {}", line_number, char);
                            exit_code = 65;
                        }
                    }
                }
                println!("EOF  null");
                if exit_code != 0 {
                    exit(exit_code)
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
