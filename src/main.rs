/*
 * 
 */

mod mak_lexer;
mod mak_tokens;
mod mak_expression;
mod mak_parser;
mod mak_basic_block;
mod mak_control_flow;

use std::fs;

use crate::mak_lexer::*;
use crate::mak_expression::*;
use crate::mak_parser::*;

fn main() {
    /*
    let test: String = 
        "
        fn main () {
            let a = 3;

            while ( a <= 5 ) {
                a = a + 1;
            }
        }
        ".to_string();
    */
    
    let args: Vec<_> = std::env::args().collect();
    let example = fs::read_to_string(args[1].clone()).unwrap();
    let tokens = Lexer::from_string(&example);
    dbg!(&tokens);
    let functions = Parser::get_functions(tokens.tokens_.clone());
    dbg!(&functions);
}
