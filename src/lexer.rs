#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
    Identifier(String),
    LeftBrack,
    RightBrack,
    Comma,
    Assign,
    EOL,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        if self.pos < self.input.len() {
            let current_char = self.input[self.pos];
            self.pos += 1;
            Some(current_char)
        } else {
            None
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.advance() {
            match c {
                '0'..='9' => {
                    let mut num_str = String::new();
                    num_str.push(c);
                    while let Some(digit @ '0'..='9') = self.peek() {
                        num_str.push(*digit);
                        self.advance();
                    }
                    let num: f64 = num_str.parse().unwrap();
                    tokens.push(Token::Number(num));
                }
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                ',' => tokens.push(Token::Comma),
                '=' => tokens.push(Token::Assign),
                '{' => tokens.push(Token::LeftBrack),
                '}' => tokens.push(Token::RightBrack),
                ';' => tokens.push(Token::EOL),
                _ if c.is_alphabetic() => {
                    let mut identifier = String::new();
                    identifier.push(c);
                    while let Some(next_char) = self.peek() {
                        if next_char.is_alphanumeric() || *next_char == '_' {
                            identifier.push(*next_char);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Identifier(identifier));
                }
                _ => {}
            }
        }
        tokens.push(Token::EOF);
        tokens
    }

    pub fn peek(&self) -> Option<&char> {
        if self.pos < self.input.len() {
            Some(&self.input[self.pos])
        } else {
            None
        }
    }
}