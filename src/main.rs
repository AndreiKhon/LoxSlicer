use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Token {
    _type: TokenType,
    lexeme: String,
    line: usize,
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

fn check_reserved(word: &String) -> TokenType {
    match word.as_str() {
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
    }
}

pub struct Scanner {}

impl Scanner {
    pub fn scan(file_contents: String) -> (Vec<Token>, i32) {
        let mut exit_code = 0;
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        let mut line_number = 1;
        if !file_contents.is_empty() {
            let mut file_content_chars = file_contents.chars().peekable();

            while let Some(char) = file_content_chars.next() {
                match char {
                    '(' => tokens.push(Token {
                        _type: TokenType::LeftParen,
                        lexeme: "(".to_string(),
                        line: line_number,
                    }),
                    ')' => tokens.push(Token {
                        _type: TokenType::RightParen,
                        lexeme: ")".to_string(),
                        line: line_number,
                    }),
                    '{' => tokens.push(Token {
                        _type: TokenType::LeftBrace,
                        lexeme: "{".to_string(),
                        line: line_number,
                    }),
                    '}' => tokens.push(Token {
                        _type: TokenType::RightBrace,
                        lexeme: "}".to_string(),
                        line: line_number,
                    }),
                    ',' => tokens.push(Token {
                        _type: TokenType::Comma,
                        lexeme: ",".to_string(),
                        line: line_number,
                    }),
                    '.' => tokens.push(Token {
                        _type: TokenType::Dot,
                        lexeme: ".".to_string(),
                        line: line_number,
                    }),
                    '-' => tokens.push(Token {
                        _type: TokenType::Minus,
                        lexeme: "-".to_string(),
                        line: line_number,
                    }),
                    '+' => tokens.push(Token {
                        _type: TokenType::Plus,
                        lexeme: "+".to_string(),
                        line: line_number,
                    }),
                    ';' => tokens.push(Token {
                        _type: TokenType::Semicolon,
                        lexeme: ";".to_string(),
                        line: line_number,
                    }),
                    '*' => tokens.push(Token {
                        _type: TokenType::Star,
                        lexeme: "*".to_string(),
                        line: line_number,
                    }),
                    '=' => match file_content_chars.peek() {
                        Some('=') => {
                            file_content_chars.next();
                            tokens.push(Token {
                                _type: TokenType::EqualEqual,
                                lexeme: "==".to_string(),
                                line: line_number,
                            });
                        }
                        _ => tokens.push(Token {
                            _type: TokenType::Equal,
                            lexeme: "=".to_string(),
                            line: line_number,
                        }),
                    },
                    '!' => match file_content_chars.peek() {
                        Some('=') => {
                            file_content_chars.next();
                            tokens.push(Token {
                                _type: TokenType::BangEqual,
                                lexeme: "!=".to_string(),
                                line: line_number,
                            });
                        }
                        _ => tokens.push(Token {
                            _type: TokenType::Bang,
                            lexeme: "!".to_string(),
                            line: line_number,
                        }),
                    },
                    '<' => match file_content_chars.peek() {
                        Some('=') => {
                            file_content_chars.next();
                            tokens.push(Token {
                                _type: TokenType::LessEqual,
                                lexeme: "<=".to_string(),
                                line: line_number,
                            });
                        }
                        _ => tokens.push(Token {
                            _type: TokenType::Less,
                            lexeme: "<".to_string(),
                            line: line_number,
                        }),
                    },
                    '>' => match file_content_chars.peek() {
                        Some('=') => {
                            file_content_chars.next();
                            tokens.push(Token {
                                _type: TokenType::GreaterEqual,
                                lexeme: ">=".to_string(),
                                line: line_number,
                            });
                        }
                        _ => tokens.push(Token {
                            _type: TokenType::Greater,
                            lexeme: ">".to_string(),
                            line: line_number,
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
                            line: line_number,
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
                                line: line_number,
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
                            line: line_number,
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
                        let token_type = check_reserved(&string);
                        tokens.push(Token {
                            _type: token_type,
                            lexeme: string,
                            line: line_number,
                        });
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
            line: line_number,
        });

        return (tokens, exit_code);
    }
}

enum Expression {
    Literal(Token),
    Unary(Token, Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Grouping(Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Literal(value) => {
                let literal = match &value._type {
                    TokenType::StringLiteral(value) => value.to_string(),
                    TokenType::Number(value) => {
                        let integer: f64 = (*value as i64) as f64;
                        if integer.to_bits() == value.to_bits() {
                            format!("{}.0", integer)
                        } else {
                            value.to_string()
                        }
                    }
                    _ => format!("{}", value.lexeme),
                };
                write!(f, "{}", literal)
            }
            Expression::Unary(operator, expression) => {
                write!(f, "({} {})", operator.lexeme, expression)
            }
            Expression::Binary(left, operator, right) => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expression::Grouping(expression) => write!(f, "(group {})", expression),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

type ParserResult = Result<Expression, String>;

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn parse(&mut self) -> ParserResult {
        return self.expression();
    }

    fn expression(&mut self) -> ParserResult {
        return self.equality();
    }

    fn equality(&mut self) -> ParserResult {
        let mut lhs = self.comparison()?;

        while matches!(
            self.peek()._type,
            TokenType::BangEqual | TokenType::EqualEqual
        ) {
            let operator = self.advance();
            let rhs = self.comparison()?;
            lhs = Expression::Binary(Box::new(lhs), operator, Box::new(rhs));
        }

        return Ok(lhs);
    }

    fn comparison(&mut self) -> ParserResult {
        let mut lhs = self.term()?;

        while matches!(
            self.peek()._type,
            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual
        ) {
            let operator = self.advance();
            let rhs = self.term()?;
            lhs = Expression::Binary(Box::new(lhs), operator, Box::new(rhs));
        }

        return Ok(lhs);
    }

    fn term(&mut self) -> ParserResult {
        let mut lhs = self.factor()?;

        while matches!(self.peek()._type, TokenType::Minus | TokenType::Plus) {
            let operator = self.advance();
            let rhs = self.factor()?;
            lhs = Expression::Binary(Box::new(lhs), operator, Box::new(rhs));
        }

        return Ok(lhs);
    }

    fn factor(&mut self) -> ParserResult {
        let mut left = self.unary()?;

        while matches!(self.peek()._type, TokenType::Slash | TokenType::Star) {
            let operator = self.advance();
            let right = self.unary()?;
            left = Expression::Binary(Box::new(left), operator, Box::new(right));
        }

        return Ok(left);
    }

    fn unary(&mut self) -> ParserResult {
        if matches!(self.peek()._type, TokenType::Bang | TokenType::Minus) {
            let operator = self.advance();
            let right = self.unary()?;

            return Ok(Expression::Unary(operator, Box::new(right)));
        }

        return self.primary();
    }

    fn primary(&mut self) -> ParserResult {
        if matches!(
            self.peek()._type,
            TokenType::False
                | TokenType::True
                | TokenType::Nil
                | TokenType::Number(_)
                | TokenType::StringLiteral(_)
        ) {
            return Ok(Expression::Literal(self.advance()));
        }

        if matches!(self.peek()._type, TokenType::LeftParen) {
            self.advance();
            let expression = self.expression()?;

            if !matches!(self.peek()._type, TokenType::RightParen) {
                return Err(Parser::error(
                    self.peek().clone(),
                    "Expect ')' after expression".to_string(),
                ));
            }
            self.advance();
            return Ok(Expression::Grouping(Box::new(expression)));
        }
        return Err(Parser::error(
            self.peek().clone(),
            "Unknown Error".to_string(),
        ));
    }

    fn error(token: Token, message: String) -> String {
        match token._type {
            TokenType::Eof => token.line.to_string() + " at end. " + &message,
            _ => token.line.to_string() + " at '" + &token.lexeme + "' " + &message,
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return matches!(self.peek()._type, TokenType::Eof);
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }
}

pub struct Interpreter {}

#[derive(PartialEq)]
enum Object {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Nil => writeln!(f, "nil"),
            Object::Boolean(b) => writeln!(f, "{}", b),
            Object::Number(n)=> writeln!(f, "{}", n),
            Object::String(s) => writeln!(f, "{}", s)
        }
    }
}

type InterpreterResult = Result<Object, String>;

impl Interpreter {
    fn evaluate(&self, expression: &Expression) -> InterpreterResult {
        match expression {
            Expression::Literal(value) => match &value._type {
                TokenType::Nil => return Ok(Object::Nil),
                TokenType::True => return Ok(Object::Boolean(true)),
                TokenType::False => return Ok(Object::Boolean(false)),
                TokenType::Number(n) => return Ok(Object::Number(*n)),
                TokenType::StringLiteral(s) => return Ok(Object::String(s.to_string())),
                a => return Err(format!("'{}': Wrong literal", a)),
            },
            Expression::Unary(operator, value) => {
                let right = self.evaluate(value)?;
                match operator._type {
                    TokenType::Minus => match right {
                        Object::Number(n) => return Ok(Object::Number(-n)),
                        _ => Err(format!("'{}': Operand must be a number", operator._type)),
                    },
                    TokenType::Bang => return Ok(Object::Boolean(!Interpreter::is_truthy(&right))),
                    _ => Err(format!("'{}': Wrong unari operator", operator._type)),
                }
            }
            Expression::Binary(lhs, operator, rhs) => {
                let lhs = self.evaluate(lhs)?;
                let rhs = self.evaluate(rhs)?;

                match operator._type {
                    TokenType::Minus => match (lhs, rhs) {
                        (Object::Number(n), Object::Number(m)) => return Ok(Object::Number(n - m)),
                        _ => return Err(format!("'{}': Operands must be numbers", operator._type)),
                    },
                    TokenType::Slash => match (lhs, rhs) {
                        (Object::Number(n), Object::Number(m)) => return Ok(Object::Number(n / m)),
                        _ => return Err(format!("'{}': Operands must be numbers", operator._type)),
                    },
                    TokenType::Star => match (lhs, rhs) {
                        (Object::Number(n), Object::Number(m)) => return Ok(Object::Number(n * m)),
                        _ => return Err(format!("'{}': Operands must be numbers", operator._type)),
                    },
                    TokenType::Plus => match (lhs, rhs) {
                        (Object::Number(n), Object::Number(m)) => return Ok(Object::Number(n + m)),
                        (Object::String(s), Object::String(t)) => {
                            return Ok(Object::String(s + &t))
                        }
                        _ => {
                            return Err(format!(
                                "'{}': Operands must be numbers or strings",
                                operator._type
                            ))
                        }
                    },
                    TokenType::Greater => match (lhs, rhs) {
                        (Object::Number(n), Object::Number(m)) => {
                            return Ok(Object::Boolean(n > m))
                        }
                        _ => return Err(format!("'{}': Operands must be numbers", operator._type)),
                    },
                    TokenType::GreaterEqual => match (lhs, rhs) {
                        (Object::Number(n), Object::Number(m)) => {
                            return Ok(Object::Boolean(n >= m))
                        }
                        _ => {
                            let err = format!("'{}': Operands must be numbers", operator._type);
                            eprintln!("{}", err);
                            return Err(err);
                        }
                    },
                    TokenType::Less => match (lhs, rhs) {
                        (Object::Number(n), Object::Number(m)) => {
                            return Ok(Object::Boolean(n < m))
                        }
                        _ => {
                            let err = format!("'{}': Operands must be numbers", operator._type);
                            eprintln!("{}", err);
                            return Err(err);
                        }
                    },
                    TokenType::LessEqual => match (lhs, rhs) {
                        (Object::Number(n), Object::Number(m)) => {
                            return Ok(Object::Boolean(n <= m))
                        }
                        _ => {
                            let err = format!("'{}': Operands must be numbers", operator._type);
                            eprintln!("{}", err);
                            return Err(err);
                        }
                    },
                    TokenType::BangEqual => {
                        return Ok(Object::Boolean(!Interpreter::is_equal(&lhs, &rhs)))
                    }
                    TokenType::EqualEqual => {
                        return Ok(Object::Boolean(Interpreter::is_equal(&lhs, &rhs)))
                    }
                    _ => return Err(format!("'{}': Wrong binary expression", operator._type)),
                }
            }
            Expression::Grouping(value) => return self.evaluate(value),
        }
    }

    fn is_truthy(object: &Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Boolean(b) => *b,
            _ => true,
        }
    }

    fn is_equal(lhs: &Object, rhs: &Object) -> bool {
        match (lhs, rhs) {
            (Object::Nil, Object::Nil) => return true,
            (Object::Nil, _) => return false,
            (a, b) => return a == b,
        }
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
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let (tokens, exit_code) = Scanner::scan(file_contents);

            if exit_code != 0 {
                exit(exit_code)
            }

            let expression = Parser::new(tokens).parse();
            match expression {
                Ok(n) => println!("{}", n),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(65);
                }
            }
        }
        "evaluate" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let (tokens, exit_code) = Scanner::scan(file_contents);

            if exit_code != 0 {
                exit(exit_code)
            }

            let expression = Parser::new(tokens).parse();
            match &expression {
                Ok(n) => (),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(65);
                }
            }

            let expression = expression.unwrap();
            let interpreter = Interpreter{};
            let value = interpreter.evaluate(&expression);
            match &value {
                Ok(n) => println!("{}", n),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(65);
                }
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
