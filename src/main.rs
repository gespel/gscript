#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::collections::HashMap;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;

mod script_language;
mod lexer;
mod interpreter;

fn main() {
    let mut l = Lexer::new("
        y2 = 1 + 1;
        y3 = y2 + ((4 + 1) - 2);
        x = y2 + y3;
        x2 = 1;

        fn test() {
            x = 0;
            y = 1;
        }
        fn test2() {
            x1 = 2;
        }
        test();
        test();
        test2();
    ");
    //println!("{:?}", l.tokenize());
    let mut i = Interpreter::new(l.tokenize(), HashMap::new());
    i.interpret();
    i.print_debug();
}
