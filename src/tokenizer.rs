use crate::tokenizer::Token::EOS;

enum Token{
    EOS,
    ZERO,
    ONE,
    TWO,
    OPEN,
    CLOSE,
    PLUS,
    MULT,
}

fn show_tok(t: Token) -> &'static str {
    match t{
        Token::EOS => "EOS", // {return "EOS"}
        Token::ZERO => "ZERO",
        Token::ONE => "ONE",
        Token::TWO => "TWO",
        Token::OPEN => "OPEN",
        Token::CLOSE => "CLOSE",
        Token::PLUS => "PLUS",
        Token::MULT => "MULT"
    }
}


struct Tokenizer {
    pos: i32,
    s: String,
    token: Token
}


impl Tokenizer {
    fn new(text: &str) -> Tokenizer {
        Tokenizer {
            pos: 0,
            s: text.parse().unwrap(),
            token: next(),
        }
    }


    pub fn next(mut self) -> Token {
        if self.s.len() <= self.pos as usize {
            return Token::EOS;
        }

        return match self.s[self.pos] {
            '0' => {
                self.pos + 1;
                Token::ZERO
            },
            '1' => {
                self.pos + 1;
                Token::ONE
            },
            '2' => {
                self.pos + 1;
                Token::TWO
            },
            '(' => {
                self.pos + 1;
                Token::OPEN
            },
            ')' => {
                self.pos + 1;
                Token::CLOSE
            },
            '+' => {
                self.pos + 1;
                Token::PLUS
            },
            '*' => {
                self.pos + 1;
                Token::MULT
            },
            __ => {
                self.pos + 1;
                ()
            }  //rest of symbols
        }

    }

    fn next_token(&self){
        token = next();
    }


    pub fn scan(&self) -> Vec<Token> {

        let mut v: Vec<Token> = vec![];
        let mut t: Token;

        while t!= EOS{
            t = next();
            v.push(t);
        }
        return v;
    }


    pub fn show(&self) -> String{
        let mut v: Vec<Token> = self.scan();
        let mut s: String = "".to_string();

        for n in 0..v.size(){
            s = s + show_tok(v[n]);

            if n + 1 < v.size() {
                s = s + ";";
            }
        }
        return s;
    }
}