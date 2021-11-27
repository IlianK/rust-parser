use crate::ast::Exp;

mod ast;
mod tokenizer;
mod parser;



fn display(e: Option<Exp>){
    if e.is_none() {
        println!("nothing \n");
    }
    else{
        let str: String = e.unwrap().pretty() + "\n";
        println!("{}", str);
    }
}

fn display_parsing(to_parse: &str) {
    let parser = parser::Parser::new(to_parse).parse();
    display(parser);
}

fn test_parser_good(){

    display_parsing("1");

    display_parsing("1 + 0");

    display_parsing("1 + (0)");

    display_parsing("1 + 2 * 0");

    display_parsing("1 * 2 + 0");

    display_parsing("(1 + 2) * 2");

    display_parsing("(1 * 2) * 0");

    display_parsing("(1 + 2) * 0 + 2");
}

fn test_parser(){
    test_parser_good();
}

fn main() {
    //let bt = backtrace::Backtrace::new();

    test_parser();

    //println!("{:?}", bt);
}

