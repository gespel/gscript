use crate::interpreter::Interpreter;
use crate::lexer::Lexer;

mod script_language;
mod lexer;
mod interpreter;

fn main() {
    let mut l = Lexer::new("
        y2 = 2
        y3 = 5
        fn test() {
            x = 0
            y = 1
        }
        fn test2() {
            x1 = 2
        }
        y2 = y3 + 3
    ");
    //println!("{:?}", l.tokenize());
    let mut i = Interpreter::new(l.tokenize());
    i.interpret();
    i.print_debug();
}
