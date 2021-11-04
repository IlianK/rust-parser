use crate::ast::Exp;

mod ast;
mod tokenizer;
mod parser;


fn display(e: Option<Box<dyn Exp>>){
    if e.is_none() {
        println!("nothing \n");
    }
    else{
        let str: String = e.unwrap().pretty() + "\n";
        println!("{}", str);
    }
}

fn test_parser_good(){
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

fn test_parser(){
    test_parser_good();
}

fn main() {
    test_parser();
}

