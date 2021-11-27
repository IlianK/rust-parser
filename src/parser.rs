use crate::ast::Exp;
use crate::tokenizer::*;

pub struct Parser {
    t: Tokenizer
}

impl Parser {
    pub fn new(string_to_parse: &str) -> Parser{
        Parser {
            t: Tokenizer::helper(string_to_parse)
        }
    }

    pub fn parse(&mut self) -> Option<Exp>{
        let e = Parser::parse_e(self);
        return e;
    }


    // E  ::= T E'
    fn parse_e(&mut self) -> Option<Exp> {
        let t: Option<Exp> = Parser::parse_t(self);
        if t.is_none() {
            return t;
        }
        return Parser::parse_e2(self, t.unwrap());
    }


    // E' ::= + T E' |
    fn parse_e2(&mut self, left: Exp) -> Option<Exp>{

        return match self.t.token {
            Token::PLUS => {
                self.t.next_token();

                let right: Option<Exp> = Parser::parse_t(self);  //get next number to append

                if right.is_none() {
                    return right;
                }

                Parser::parse_e2(self, Exp::Plus{ e1: Box::from(left), e2: Box::from(right.unwrap()), b: false })
            }

            _ => {
                Some(left)
            }
        }
    }

    // T  ::= F T'
    fn parse_t(&mut self) -> Option<Exp> {
        let f: Option<Exp> = Parser::parse_f(self);

        if f.is_none() {
            return f;
        }

        return Parser::parse_t2(self,f.unwrap());
    }

    // T' ::= * F T' |
    fn parse_t2(&mut self, left: Exp) -> Option<Exp> {
        return match self.t.token {
            Token::MULT => {
                self.t.next_token();
                let right: Option<Exp> = Parser::parse_f(self);

                if right.is_none() {
                    return right;
                }

                Parser::parse_t2(self, Exp::Mult { e1: Box::from(left), e2: Box::from(right.unwrap()), b: false })
            }
            _ => {
                Some(left)
            }
        }
    }


    // F ::= N | (E)
    fn parse_f(& mut self) -> Option<Exp> {
        return match &self.t.token {
            Token::ZERO => {
                self.t.next_token();
                Some(Exp::Int{val: 0, b: false })
            },
            Token::ONE => {
                self.t.next_token();
                Some(Exp::Int{val: 1, b: false })
            },
            Token::TWO => {
                self.t.next_token();
                Some(Exp::Int{val: 2, b: false })
            },
            Token::OPEN => {
                self.t.next_token();
                let e: Option<Exp> = Parser::parse_e(self);
                if e.is_none() {
                    return e;
                }

                return match self.t.token {
                    Token::CLOSE => {
                        self.t.next_token();
                        e
                    }
                    _ => {
                        let x: Option<Exp>  = None;
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