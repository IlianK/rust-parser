use std::any::type_name;
use crate::ast::{Exp, Int, Mult, Plus};
use crate::utility::Optional;
use crate::tokenizer::Token;
use crate::tokenizer::Tokenizer;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


pub(crate) struct Parser {
    t: Tokenizer
}

impl Parser {
    pub(crate) fn new(mut self, string_to_parse: &str) -> Parser{
        Parser {
            t: Tokenizer::new(self.t,string_to_parse)
        }
    }

    pub(crate) fn parse(self) -> Optional<Box<dyn Exp>> {
        let mut e = Parser::parse_e(self);
        return e;
    }


    // E  ::= T E'
    fn parse_e(self) -> Optional<Box<dyn Exp>> {
        let t: Optional<Box<dyn Exp>> = Parser::parse_t(self);
        if t.is_nothing() {
            return t;
        }
        return Parser::parse_e2(self, t.from_just());
    }


    // E' ::= + T E' |
    fn parse_e2(self, left: Box<dyn Exp>) -> Optional<Box<dyn Exp>> {

        if self.t.token == Token::PLUS {
            self.t.next_token();

            let right: Optional<Box<dyn Exp>> = Parser::parse_t(self);  //get next number to append

            if right.is_nothing() {
                return right;
            }

            return Parser::parse_e2(self,Plus{e1: left, e2: right.fromJust()})

        }
        return Optional::just(left);
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
    fn parse_t(self) -> Optional<Box<dyn Exp>> {
        let f: Optional<Box<dyn Exp>> = Parser::parse_f(self);

        if f.is_nothing() {
            return f;
        }

        return Parser::parse_t2(self,f.from_just());
    }

    // T' ::= * F T' |
    fn parse_t2(self, left: Box<dyn Exp>) -> Optional<Box<dyn Exp>> {
        if self.t.token == Token::MULT{
            self.t.next_token();
            let right: Optional<Box<dyn Exp>> = Parser::parse_f(self);

            if right.is_nothing() {
                return right;
            }

            return Parser::parse_t2(self,Mult{e1: left, e2:  right.from_just()})
        }
        return Optional::just(left);
    }


    // F ::= N | (E)
    fn parse_f(self) -> Optional<Box<dyn Exp>> {
        return match self.t.token {
            Token::ZERO => {
                self.t.next_token();
                Optional::just(Int{val: 0 })
            },
            Token::ONE => {
                self.t.next_token();
                Optional::just(Int{val: 1 })
            },
            Token::TWO => {
                self.t.next_token();
                Optional::just(Int{val: 2 })
            },
            Token::OPEN => {
                self.t.next_token();
                let e: Optional<Box<dyn Exp>> = Parser::parse_e(self);
                if e.is_nothing() {
                    return e;
                }
                if self.t.token != Token::CLOSE {
                    return Optional::nothing();
                }
                self.t.next_token();
                e
            }
            __ => {
                Optional::nothing()
            }
        }
    }
}

