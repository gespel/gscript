use std::collections::HashMap;
use std::process::exit;
use crate::lexer::Token;

pub struct Interpreter {
    tokens: Vec<Token>,
    line_index: usize,
    functions: HashMap<String, Vec<Token>>,
    variables: HashMap<String, f64>
}

impl Interpreter {
    pub fn new(tokens: Vec<Token>) -> Interpreter {
        Interpreter {
            tokens,
            line_index: 0,
            functions: HashMap::new(),
            variables: HashMap::new()
        }
    }

    pub fn interpret(&mut self) {
        if let Token::Identifier(ident) = self.tokens[self.line_index].clone() {
            self.line_index += 1;
            match ident.as_str() {
                "fn" => {
                    if let Token::Identifier(fn_name) = self.tokens[self.line_index].clone() {
                        self.line_index += 1;
                        self.eat(Token::LeftParen);
                        self.eat(Token::RightParen);
                        self.eat(Token::LeftBrack);
                        let mut fn_tokens: Vec<Token> = Vec::new();
                        loop {
                            fn_tokens.push(self.tokens[self.line_index].clone());
                            if let Token::RightBrack = self.tokens[self.line_index+1] {
                                break;
                            }
                            self.line_index += 1;
                        }
                        self.line_index += 1;
                        self.functions.insert(fn_name, fn_tokens);
                    }
                }
                var_name => {
                    if let Token::Assign = self.tokens[self.line_index].clone() {
                        self.line_index += 1;
                        if let Token::Number(var_value) = self.tokens[self.line_index].clone() {
                            self.variables.insert(var_name.parse().unwrap(), var_value);
                        }
                        if let Token::Identifier(var2_name) = self.tokens[self.line_index].clone() {
                            self.line_index += 1;
                            self.eat(Token::Plus);
                            if let Token::Number(add) = self.tokens[self.line_index].clone() {
                                self.line_index += 1;
                                self.variables.insert(var_name.parse().unwrap(), self.variables.get(&var2_name).unwrap().clone() + add);
                            }

                        }
                    }
                }
            }
        }
        self.line_index += 1;
        if self.line_index < self.tokens.len() - 1 {
            self.interpret();
        }
    }

    fn eat(&mut self, t: Token) {
        if self.tokens[self.line_index].clone() == t {
            self.line_index += 1;
        }
        else {
            println!("SYNTAX ERRROR!");
            exit(-1);
        }
    }

    pub fn print_debug(&self) {
        println!("defined functions:");
        println!("{:?}", self.functions);
        println!("defined variables:");
        println!("{:?}", self.variables);
    }
}