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

    fn terminal(&mut self) -> f64 {
        match self.tokens[self.line_index].clone() {
            Token::Number(nr) => {
                return nr
            }
            Token::Identifier(var_name) => {
                self.variables.get(&var_name).unwrap().clone()
            }
            _ => {
                panic!("Terminal parsing Error! Expected terminal but got {:?} instead!", self.tokens[self.line_index].clone());
            }
        }
    }

    fn expression(&mut self) -> f64 {
        let mut result = self.terminal();
        self.line_index += 1;
        loop {
            if self.peak(Token::EOL) {
                break;
            }
            self.eat(Token::Plus);
            let result2 = self.terminal();
            self.line_index += 1;
            result += result2;
        }
        result
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
        if let Token::Identifier(ident) = self.tokens[self.line_index].clone() {
            self.line_index += 1;
            match ident.as_str() {
                //function definition
                "fn" => {
                    self.function_definition();
                }

                var_name => {
                    if self.peak(Token::Assign) {
                        self.eat(Token::Assign);
                        let result = self.expression();
                        self.variables.insert(var_name.parse().unwrap(), result);
                        //variable assignment
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