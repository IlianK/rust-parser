use crate::ast::Exp;


mod ast;
mod tokenizer;
mod parser;


fn display(e: Option<Box<Exp>>){
    if e.is_none() {
        println!("nothing \n");
    }
    else{
        let str: String = e.unwrap().pretty() + "\n";
        println!("{}", str);
    }
}

fn test_parser_good(){

    display(parser::Parser::new("1").parse());

    display(parser::Parser::new("1 + 0").parse());

    display(parser::Parser::new("1 + (0)").parse());

    display(parser::Parser::new("1 + 2 * 0").parse());

    display(parser::Parser::new("1 * 2 + 0").parse());


    display(parser::Parser::new("(1 + 2) * 2").parse());

    display(parser::Parser::new("(1 + 2) * 0").parse());

    display(parser::Parser::new("(1 + 2) * 0 + 2").parse());
}

fn test_parser(){
    test_parser_good();
}

fn main() {
    //let bt = backtrace::Backtrace::new();

    // do_some_work();
    test_parser();

    //println!("{:?}", bt);
}

