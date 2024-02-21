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

    fn function_definition(&mut self) {
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
        self.line_index += 1;
    }

    pub fn interpret(&mut self) {
        println!("{:?}", self.tokens[self.line_index].clone());
        if let Token::Identifier(ident) = self.tokens[self.line_index].clone() {
            self.line_index += 1;
            match ident.as_str() {
                "fn" => {
                    self.function_definition();
                }
                var_name => {
                    //variable assignment
                    if self.peak(Token::Assign) {
                        self.eat(Token::Assign);
                        match self.tokens[self.line_index].clone() {
                            Token::Number(assign_nr) => {
                                self.variables.insert(var_name.parse().unwrap(), assign_nr);
                            }
                            Token::Plus => {

                            }
                            Token::Minus => {}
                            Token::Multiply => {}
                            Token::Divide => {}
                            Token::LeftParen => {}
                            Token::RightParen => {}
                            Token::Identifier(var2) => {
                                self.line_index += 1;
                                self.eat(Token::Plus);
                                match self.tokens[self.line_index].clone() {
                                    Token::Number(add_nr) => {
                                        self.variables.insert(var_name.parse().unwrap(), self.variables.get(&var2).unwrap().clone() + add_nr);
                                    }
                                    Token::Plus => {}
                                    Token::Minus => {}
                                    Token::Multiply => {}
                                    Token::Divide => {}
                                    Token::LeftParen => {}
                                    Token::RightParen => {}
                                    Token::Identifier(var3) => {
                                        self.variables.insert(
                                            var_name.parse().unwrap(),
                                            self.variables.get(&var2).unwrap().clone() + self.variables.get(&var3).unwrap().clone()
                                        );
                                    }
                                    Token::LeftBrack => {}
                                    Token::RightBrack => {}
                                    Token::Comma => {}
                                    Token::Assign => {}
                                    Token::EOL => {}
                                    Token::EOF => {}
                                }
                            }
                            Token::LeftBrack => {}
                            Token::RightBrack => {}
                            Token::Comma => {}
                            Token::Assign => {}
                            Token::EOL => {}
                            Token::EOF => {}
                        }
                        self.line_index += 1;
                        /*if let Token::Number(var_value) = self.tokens[self.line_index].clone() {
                            self.variables.insert(var_name.parse().unwrap(), var_value);
                        }
                        if let Token::Identifier(var2_name) = self.tokens[self.line_index].clone() {
                            self.line_index += 1;
                            self.eat(Token::Plus);
                            if let Token::Number(add) = self.tokens[self.line_index].clone() {
                                self.line_index += 1;
                                self.variables.insert(var_name.parse().unwrap(), self.variables.get(&var2_name).unwrap().clone() + add);
                            }

                        }*/
                    }
                    //function call
                    if self.peak(Token::LeftParen) {

                    }
                    self.eat(Token::EOL);
                }
            }
        }

        if self.line_index < self.tokens.len() - 1 {
            self.interpret();
        }
    }

    fn eat(&mut self, t: Token) {
        if self.tokens[self.line_index].clone() == t {
            self.line_index += 1;
        }
        else {
            println!("SYNTAX ERRROR! Expected {:?} at index {} (found {:?})", t, self.line_index, self.tokens[self.line_index].clone());
            exit(-1);
        }
    }
    fn peak(&mut self, t: Token) -> bool {
        if self.tokens[self.line_index].clone() == t {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn print_debug(&self) {
        println!("defined functions:");
        println!("{:?}", self.functions);
        println!("defined variables:");
        println!("{:?}", self.variables);
    }
}