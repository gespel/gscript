use crate::interpreter::Interpreter;
use crate::lexer::Lexer;

mod script_language;
mod lexer;
mod interpreter;

fn main() {
    let mut l = Lexer::new("xasd = 3;xa = 2;s = 1;");
    let mut i = Interpreter::new(l.tokenize());
    i.interpret();
}
