#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;

mod script_language;
mod lexer;
mod interpreter;

fn main() {
    let mut l = Lexer::new("
        y2 = 0;
        y3 = 5;
        fn test() {
            x = 0
            y = 1
        }
        fn test2() {
            x1 = 2
        }
        x = 3;
        x = y2 + y3;
    ");
    //println!("{:?}", l.tokenize());
    let mut i = Interpreter::new(l.tokenize());
    i.interpret();
    i.print_debug();
}
