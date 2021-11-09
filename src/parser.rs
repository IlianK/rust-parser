use crate::ast::Exp;
//use std::any::type_name;
use crate::tokenizer::Token;
use crate::tokenizer::Tokenizer;
use crate::Exp::Int;
use crate::Exp::Plus;
use crate::Exp::Mult;

/*
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
*/

pub struct Parser {
    t: Tokenizer
}

impl Parser {
    pub fn new(string_to_parse: &str) -> Parser{
        Parser {
            t: Tokenizer::helper(string_to_parse)
        }
    }

    pub fn parse(&mut self) -> Option<Box<Exp>>{
        let e = Parser::parse_e(self);
        return e;
    }


    // E  ::= T E'
    fn parse_e(&mut self) -> Option<Box<Exp>> {
        let t: Option<Box<Exp>> = Parser::parse_t(self);
        if t.is_none() {
            return t;
        }
        return Parser::parse_e2(self, t.unwrap());
    }


    // E' ::= + T E' |
    fn parse_e2(&mut self, left: Box<Exp>) -> Option<Box<Exp>>{

        return match self.t.token {
            Token::PLUS => {
                self.t.next_token();

                let right: Option<Box<Exp>> = Parser::parse_t(self);  //get next number to append

                if right.is_none() {
                    return right;
                }

                Parser::parse_e2(self, Box::new(Plus{ e1: left, e2: right.unwrap(), been_there: false }))
            }

            _ => {
                Some(left)
            }
        }
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
    fn parse_t(&mut self) -> Option<Box<Exp>> {
        let f: Option<Box<Exp>> = Parser::parse_f(self);

        if f.is_none() {
            return f;
        }

        return Parser::parse_t2(self,f.unwrap());
    }

    // T' ::= * F T' |
    fn parse_t2(&mut self, left: Box<Exp>) -> Option<Box<Exp>> {
        return match self.t.token {
            Token::MULT => {
                self.t.next_token();
                let right: Option<Box<Exp>> = Parser::parse_f(self);

                if right.is_none() {
                    return right;
                }

                Parser::parse_t2(self, Box::new(Mult { e1: left, e2: right.unwrap(), been_there: false }))
            }
            _ => {
                Some(left)
            }
        }
    }


    // F ::= N | (E)
    fn parse_f(& mut self) -> Option<Box<Exp>> {
        return match &self.t.token {
            Token::ZERO => {
                self.t.next_token();
                Some(Box::new(Int{val: 0, been_there: false }))
            },
            Token::ONE => {
                self.t.next_token();
                Some(Box::new(Int{val: 1, been_there: false }))
            },
            Token::TWO => {
                self.t.next_token();
                Some(Box::new(Int{val: 2, been_there: false }))
            },
            Token::OPEN => {
                self.t.next_token();
                let e: Option<Box<Exp>> = Parser::parse_e(self);
                if e.is_none() {
                    return e;
                }

                return match self.t.token {
                    Token::CLOSE => {
                        self.t.next_token();
                        e
                    }
                    _ => {
                        let x: Option<Box<Exp>>  = None;
                        x
                    }
                }

            }
            __ => {
                return None;
            }
        }
    }
}

