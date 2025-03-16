use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    Identifier,

    StringLiteral(String),
    Number(f64),

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),

            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::Semicolon => write!(f, "SEMICOLON"),
            TokenType::Slash => write!(f, "SLASH"),
            TokenType::Star => write!(f, "STAR"),

            TokenType::Equal => write!(f, "EQUAL"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenType::Bang => write!(f, "BANG"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL"),
            TokenType::Less => write!(f, "LESS"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL"),
            TokenType::Greater => write!(f, "GREATER"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL"),

            TokenType::Identifier => write!(f, "IDENTIFIER"),

            TokenType::StringLiteral(_) => write!(f, "STRING"),
            TokenType::Number(_) => write!(f, "NUMBER"),

            TokenType::And => write!(f, "AND"),
            TokenType::Class => write!(f, "CLASS"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::Fun => write!(f, "FUN"),
            TokenType::For => write!(f, "FOR"),
            TokenType::If => write!(f, "IF"),
            TokenType::Nil => write!(f, "NIL"),
            TokenType::Or => write!(f, "OR"),
            TokenType::Print => write!(f, "PRINT"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::Super => write!(f, "SUPER"),
            TokenType::This => write!(f, "THIS"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::Var => write!(f, "VAR"),
            TokenType::While => write!(f, "WHILE"),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}

struct Token {
    _type: TokenType,
    lexeme: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let literal = match &self._type {
            TokenType::StringLiteral(value) => value.to_string(),
            TokenType::Number(value) => {
                let integer: f64 = (*value as i64) as f64;
                if integer.to_bits() == value.to_bits() {
                    format!("{}.0", integer)
                } else {
                    value.to_string()
                }
            }
            _ => "null".to_string(),
        };
        write!(f, "{} {} {}", self._type, self.lexeme, literal)
    }
}

fn check_reserved(word: &String) -> Token {
    let _type = match word.as_str() {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,

        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,

        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,

        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,

        _ => TokenType::Identifier,
    };
    return Token {
        _type: _type,
        lexeme: word.to_string(),
    };
}

pub struct Scanner {}

impl Scanner {
    pub fn scan(file_contents: String) -> (Vec<Token>, i32) {
        let mut exit_code = 0;
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        if !file_contents.is_empty() {
            let mut line_number = 1;
            let mut file_content_chars = file_contents.chars().peekable();

            while let Some(char) = file_content_chars.next() {
                match char {
                    '(' => tokens.push(Token {
                        _type: TokenType::LeftParen,
                        lexeme: "(".to_string(),
                    }),
                    ')' => tokens.push(Token {
                        _type: TokenType::RightParen,
                        lexeme: ")".to_string(),
                    }),
                    '{' => tokens.push(Token {
                        _type: TokenType::LeftBrace,
                        lexeme: "{".to_string(),
                    }),
                    '}' => tokens.push(Token {
                        _type: TokenType::RightBrace,
                        lexeme: "}".to_string(),
                    }),
                    ',' => tokens.push(Token {
                        _type: TokenType::Comma,
                        lexeme: ",".to_string(),
                    }),
                    '.' => tokens.push(Token {
                        _type: TokenType::Dot,
                        lexeme: ".".to_string(),
                    }),
                    '-' => tokens.push(Token {
                        _type: TokenType::Minus,
                        lexeme: "-".to_string(),
                    }),
                    '+' => tokens.push(Token {
                        _type: TokenType::Plus,
                        lexeme: "+".to_string(),
                    }),
                    ';' => tokens.push(Token {
                        _type: TokenType::Semicolon,
                        lexeme: ";".to_string(),
                    }),
                    '*' => tokens.push(Token {
                        _type: TokenType::Star,
                        lexeme: "*".to_string(),
                    }),
                    '=' => match file_content_chars.peek() {
                        Some('=') => {
                            file_content_chars.next();
                            tokens.push(Token {
                                _type: TokenType::EqualEqual,
                                lexeme: "==".to_string(),
                            });
                        }
                        _ => tokens.push(Token {
                            _type: TokenType::Equal,
                            lexeme: "=".to_string(),
                        }),
                    },
                    '!' => match file_content_chars.peek() {
                        Some('=') => {
                            file_content_chars.next();
                            tokens.push(Token {
                                _type: TokenType::BangEqual,
                                lexeme: "!=".to_string(),
                            });
                        }
                        _ => tokens.push(Token {
                            _type: TokenType::Bang,
                            lexeme: "!".to_string(),
                        }),
                    },
                    '<' => match file_content_chars.peek() {
                        Some('=') => {
                            file_content_chars.next();
                            tokens.push(Token {
                                _type: TokenType::LessEqual,
                                lexeme: "<=".to_string(),
                            });
                        }
                        _ => tokens.push(Token {
                            _type: TokenType::Less,
                            lexeme: "<".to_string(),
                        }),
                    },
                    '>' => match file_content_chars.peek() {
                        Some('=') => {
                            file_content_chars.next();
                            tokens.push(Token {
                                _type: TokenType::GreaterEqual,
                                lexeme: ">=".to_string(),
                            });
                        }
                        _ => tokens.push(Token {
                            _type: TokenType::Greater,
                            lexeme: ">".to_string(),
                        }),
                    },
                    '/' => match file_content_chars.peek() {
                        Some('/') => {
                            let mut next_char = file_content_chars.next();
                            while next_char != None && next_char != Some('\n') {
                                next_char = file_content_chars.next();
                            }
                            line_number += 1;
                        }
                        _ => tokens.push(Token {
                            _type: TokenType::Slash,
                            lexeme: "/".to_string(),
                        }),
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
                            let mut quoted: String = "\"".to_string();
                            quoted.push_str(&string);
                            quoted.push('"');
                            tokens.push(Token {
                                _type: TokenType::StringLiteral(string.clone()),
                                lexeme: quoted,
                            });
                        } else {
                            eprintln!("[line {}] Error: Unterminated string.", line_number);
                            exit_code = 65;
                        }
                    }
                    char if char.is_numeric() == true => {
                        let mut string = String::new();
                        string.push(char);
                        while let Some(next_char) = file_content_chars.peek() {
                            if next_char.is_numeric() || *next_char == '.' {
                                string.push(*next_char);
                                file_content_chars.next();
                            } else {
                                break;
                            }
                        }
                        tokens.push(Token {
                            _type: TokenType::Number(string.parse().unwrap()),
                            lexeme: string,
                        });
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
                        let token = check_reserved(&string);
                        tokens.push(token);
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
        }

        tokens.push(Token {
            _type: TokenType::Eof,
            lexeme: "".to_string(),
        });

        return (tokens, exit_code);
    }
}

fn main() {
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

            let (tokens, exit_code) = Scanner::scan(file_contents);

            for token in tokens {
                println!("{}", token);
            }

            if exit_code != 0 {
                exit(exit_code)
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
