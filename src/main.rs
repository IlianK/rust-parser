use crate::ast::Exp;

mod ast;
mod tokenizer;
mod utility;
mod parser;

use crate::utility::Optional;

fn display(e: Optional<Box<dyn Exp>>){
    if e.isNothing() {
        println!("nothing \n");
    }
    else{
        println!( (e.fromJust()).pretty() + "\n");
    }
}

fn testParserGood(){
    /*
    display(Parser("1").parse());

    display(Parser("1 + 0 ").parse());

    display(Parser("1 + (0) ").parse());

    display(Parser("1 + 2 * 0 ").parse());

    display(Parser("1 * 2 + 0 ").parse());
    */

    display(parser::Parser::new("(1 + 2)").parse());

    //display(Parser("(1 + 2) * 0 ").parse());

    //display(Parser("(1 + 2) * 0 + 2").parse());
}

fn testParser(){
    testParserGood();
}

fn main() {
    testParser();
}

