use std::any::type_name;
use crate::ast::{Exp, Int, Mult, Plus};
use crate::utility::{just, nothing};

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


struct Parser {
    t: Tokenizer
}

impl Parser {
    fn new(string_to_parse: String)-> Parser{
        Parser {
            t: Tokenizer::new(string_to_parse)
        }
    }

    fn parse(&self)->Optional<dyn Exp>{
        let mut e = parse_e();
        return e;
    }
}


// E  ::= T E'
fn parse_e()->Optional<dyn Exp>{
    let t: Optional<dyn Exp> = parse_t();
    if t.is_nothing() {
        return t;
    }
    return parse_e2(t.from_just());
}


// E' ::= + T E' |
fn parse_e2(mut left: Box<dyn Exp>) ->Optional<dyn Exp>{

    if t.token == Token::PLUS {
        t.next_token();

        let right: Optional<dyn Exp> = parse_t();  //get next number to append

        if right.is_nothing() {
            return right;
        }

        return parse_e2(new(Plus{ been_there: false, e1: left, e2: right.fromJust()}))

    }
    return just(left);
}

/*
     if(type_of(left) == PlusN<Exp> | type_of(left) == Int<Exp>){ //left only consisted of plus operands or Int operand
         //because current token is PLUS next number given in right can be appended

         let tmp = new(PlusN<Exp>{v: left});
         tmp.push(right.fromJust());

         return parseE2(tmp);
     }
     else{ //left was MultInt
         return parseE2(new(PlusN<Exp>{v: [left, right.fromJust()]}));
     }
 */


// T  ::= F T'
fn parse_t() ->Optional<dyn Exp>{
    let f: Optional<dyn Exp> = parse_f();

    if f.is_nothing() {
        return f;
    }

    return parse_t2(f.from_just());
}

// T' ::= * F T' |
fn parse_t2(left: Box<dyn Exp>) ->Optional<dyn Exp>{
    if t.token == Token::MULT{
        t.next_token();
        let right: Optional<dyn Exp> = parse_f();

        if right.is_nothing() {
            return right;
        }

        return parse_t2(new(Mult{ been_there: false, e1: left, e2:  right.from_just()}))
    }
    return just(left);
}


// F ::= N | (E)
fn parse_f() ->Optional<dyn Exp>{
    return match t.token {
        Token::ZERO => {
            t.next_token();
            just(new(Int{ been_there: false, val: 0 }))
        },
        Token::ONE => {
            t.next_token();
            just(new(Int{ been_there: false, val: 1 }))
        },
        Token::TWO => {
            t.next_token();
            just(new(Int{ been_there: false, val: 2 }))
        },
        Token::OPEN => {
            t.next_token();
            let e: Optional <dyn Exp> = parse_e();
            if e.is_nothing() {
                return e;
            }
            if t.token != Token::CLOSE {
                return nothing();
            }
            t.next_token();
            e
        }
        __ => {
            nothing()
        }
    }
}