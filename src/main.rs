use crate::ast::Exp;

mod ast;
mod tokenizer;
mod utility;
mod parser;

fn display(e: Optional<Exp>){
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

    display(Parser("(1 + 2) * 0 ").parse());

    display(Parser("(1 + 2) * 0 + 2").parse());
}

fn testParser(){
    testParserGood();
}

fn main() {
    testParser();
}

