use crate::ast::Exp;

mod ast;
mod tokenizer;
mod parser;


fn display(e: Option<Box<dyn Exp>>){
    if e.is_none() {
        println!("nothing \n");
    }
    else{
        println!( (e.is_some()).pretty() + "\n");
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

