use crate::lexer::Token;

pub struct Interpreter {
    tokens: Vec<Token>,
    index: usize,
}

impl Interpreter {
    pub fn new(tokens: Vec<Token>) -> Interpreter {
        Interpreter {
            tokens,
            index: 0
        }
    }
    pub fn interpret(&mut self) {
        let mut line: Vec<Token> = Vec::new();
        for token in self.tokens.clone() {
            if token == Token::EOL {
                self.interpret_line(line.clone());
                line.clear();
                continue;
            }
            line.push(token);
        }
    }

    fn interpret_line(&mut self, token_line: Vec<Token>) {
        println!("Interpreting line now!");
        let mut i = 0;
        if let Token::Identifier(ident) = token_line[i].clone() {
            i += 1;
            match ident.as_str() {
                "fn" => {
                    if let Token::Identifier(fn_name) = token_line[i].clone() {

                    }

                }
                _ => {

                }
            }
        }

    }
}