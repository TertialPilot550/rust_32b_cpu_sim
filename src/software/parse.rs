use super::tokenize::Token;
use crate::datatypes::{*};

/*
 * Take in a Vector of tokens, and return a protogram
 */

pub fn parse(tokens: Vec<Token>) -> Option<Protogram> {









    None
}



struct Parser {
    tokens: Vec<Token>,
    cursor: usize
}

enum Line {
    Instruction, StaticData
}

impl Parser {

    fn new(tks: Vec<Token>) -> Self {
        Parser {
            tokens: tks,
            cursor: 0
        }
        
    }

    fn read_line() -> Line {
        
    }

}