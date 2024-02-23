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
    pub fn new(tokens: Vec<Token>, args: HashMap<String, f64>) -> Interpreter {
        Interpreter {
            tokens,
            line_index: 0,
            functions: HashMap::new(),
            variables: args
        }
    }

    fn terminal(&mut self) -> f64 {
        match self.tokens[self.line_index].clone() {
            Token::Number(nr) => {
                return nr
            }
            Token::Identifier(var_name) => {
                if self.peak(Token::LeftParen, 1) {
                    0f64
                }
                else {
                    self.variables.get(&var_name).unwrap().clone()
                }

            }
            Token::LeftParen => {
                self.eat(Token::LeftParen);
                let result = self.expression();
                self.eat(Token::RightParen);
                self.decrease_line_index();
                result
            }
            _ => {
                panic!("Terminal parsing Error! Expected terminal but got {:?} instead!", self.tokens[self.line_index].clone());
            }
        }
    }

    fn expression(&mut self) -> f64 {
        let mut result = self.terminal();
        self.increase_line_index();
        loop {
            if self.peak(Token::EOL, 0) || self.peak(Token::RightParen, 0) {
                break;
            }
            match self.look(0) {
                Token::Plus => {
                    self.eat(Token::Plus);
                    let result2 = self.terminal();
                    self.increase_line_index();
                    result += result2;
                }
                Token::Minus => {
                    self.eat(Token::Minus);
                    let result2 = self.terminal();
                    self.increase_line_index();
                    result -= result2;
                }
                _ => {

                }
            }

        }
        result
    }

    fn function_definition(&mut self) {
        if let Token::Identifier(fn_name) = self.tokens[self.line_index].clone() {
            self.increase_line_index();
            self.eat(Token::LeftParen);
            self.eat(Token::RightParen);
            self.eat(Token::LeftBrack);
            let mut fn_tokens: Vec<Token> = Vec::new();
            loop {
                fn_tokens.push(self.tokens[self.line_index].clone());
                if let Token::RightBrack = self.tokens[self.line_index+1] {
                    break;
                }
                self.increase_line_index();
            }
            self.increase_line_index();
            self.functions.insert(fn_name, fn_tokens);
        }
        self.increase_line_index();
    }

    pub fn interpret(&mut self) {
        if let Token::Identifier(ident) = self.tokens[self.line_index].clone() {
            self.increase_line_index();
            match ident.as_str() {
                //function definition
                "fn" => {
                    self.function_definition();
                }

                var_name => {
                    if self.peak(Token::Assign, 0) {
                        self.eat(Token::Assign);
                        let result = self.expression();
                        self.variables.insert(var_name.parse().unwrap(), result);
                        //variable assignment
                    }
                    //function call
                    if self.peak(Token::LeftParen, 0) {
                        self.eat(Token::LeftParen);
                        self.eat(Token::RightParen);
                        self.call_function(var_name.to_string());
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
            self.increase_line_index();
        }
        else {
            println!("SYNTAX ERRROR! Expected {:?} at index {} (found {:?})", t, self.line_index, self.tokens[self.line_index].clone());
            exit(-1);
        }
    }
    fn peak(&mut self, t: Token, offset: usize) -> bool {
        if self.tokens[self.line_index + offset].clone() == t {
            return true;
        }
        else {
            return false;
        }
    }

    fn look(&mut self, offset: usize) -> Token {
        return self.tokens[self.line_index + offset].clone();
    }

    fn call_function(&mut self, function_name: String) {
        let function_vector: Vec<Token> = self.functions.get(&function_name).unwrap().clone();
        let mut i = Interpreter::new(function_vector, HashMap::new());
        i.interpret();
        i.print_debug();
    }

    fn increase_line_index(&mut self) {
        self.line_index += 1;
    }
    fn decrease_line_index(&mut self) {
        self.line_index -= 1;
    }

    pub fn print_debug(&self) {
        println!("defined functions:");
        println!("{:?}", self.functions);
        println!("defined variables:");
        println!("{:?}", self.variables);
    }
}