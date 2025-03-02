use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn check_reserved(word: &String) {
    match word.as_str() {
        "and" => println!("AND and null"),
        "class" => println!("CLASS class null"),
        "else" => println!("ELSE else null"),
        "false" => println!("FALSE false null"),

        "for" => println!("FOR for null"),
        "fun" => println!("FUN fun null"),
        "if" => println!("IF if null"),
        "nil" => println!("NIL nil null"),

        "or" => println!("OR or null"),
        "print" => println!("PRINT print null"),
        "return" => println!("RETURN return null"),
        "super" => println!("SUPER super null"),

        "this" => println!("THIS this null"),
        "true" => println!("TRUE true null"),
        "var" => println!("VAR var null"),
        "while" => println!("WHILE while null"),

        _ => println!("IDENTIFIER {} null", word),
    }
}

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
                            }
                            _ => println!("EQUAL = null"),
                        },
                        '!' => match file_content_chars.peek() {
                            Some('=') => {
                                file_content_chars.next();
                                println!("BANG_EQUAL != null");
                            }
                            _ => println!("BANG ! null"),
                        },
                        '<' => match file_content_chars.peek() {
                            Some('=') => {
                                file_content_chars.next();
                                println!("LESS_EQUAL <= null");
                            }
                            _ => println!("LESS < null"),
                        },
                        '>' => match file_content_chars.peek() {
                            Some('=') => {
                                file_content_chars.next();
                                println!("GREATER_EQUAL >= null");
                            }
                            _ => println!("GREATER > null"),
                        },
                        '/' => match file_content_chars.peek() {
                            Some('/') => {
                                let mut next_char = file_content_chars.next();
                                while next_char != None && next_char != Some('\n') {
                                    next_char = file_content_chars.next();
                                }
                                line_number += 1;
                            }
                            _ => println!("SLASH / null"),
                        },
                        '"' => {
                            let mut string = String::new();
                            let mut next_char = file_content_chars.next();
                            while next_char != None && next_char != Some('"') {
                                string.push(next_char.unwrap());
                                next_char = file_content_chars.next();
                            }
                            let mut is_closed = false;
                            match next_char {
                                None => {}
                                Some(_) => is_closed = true,
                            }
                            if is_closed == true {
                                println!("STRING \"{}\" {}", string, string);
                            } else {
                                eprintln!("[line {}] Error: Unterminated string.", line_number);
                                exit_code = 65;
                            }
                        }
                        char if char.is_numeric() == true => {
                            let mut string = String::new();
                            // let mut next_char = file_content_chars.peek();
                            string.push(char);
                            let mut is_integer = true;
                            while let Some(next_char) = file_content_chars.peek() {
                                if next_char.is_numeric() || *next_char == '.' {
                                    if *next_char == '.' {
                                        is_integer = false;
                                    }
                                    string.push(*next_char);
                                    file_content_chars.next();
                                } else {
                                    break;
                                }
                            }
                            if is_integer == true {
                                println!("NUMBER {} {}.0", string, string);
                            } else {
                                let mut short_string = string.trim_end_matches('0');
                                if short_string.chars().nth_back(0) == Some('.') {
                                    let len = short_string.len();
                                    short_string = &string[0..=len];
                                }
                                println!("NUMBER {} {}", string, short_string);
                            }
                        }
                        char if char.is_alphabetic() || char == '_' => {
                            let mut string = String::from(char);
                            while let Some(next_char) = file_content_chars.peek() {
                                if next_char.is_alphanumeric() || *next_char == '_' {
                                    string.push(*next_char);
                                    file_content_chars.next();
                                } else {
                                    break;
                                }
                            }
                            check_reserved(&string);
                        }
                        '\n' => line_number += 1,
                        '\t' | ' ' => {}
                        _ => {
                            eprintln!(
                                "[line {}] Error: Unexpected character: {}",
                                line_number, char
                            );
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
