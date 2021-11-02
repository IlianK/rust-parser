
pub(crate) enum Token{
    EOS,
    ZERO,
    ONE,
    TWO,
    OPEN,
    CLOSE,
    PLUS,
    MULT,
    DEFAULT
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
        Token::MULT => "MULT",
        Token::DEFAULT => "DEFAULT"
    }
}


pub(crate) struct Tokenizer{
    pos: usize,
    s: String,
    pub(crate) token: Token
}


impl Tokenizer {
    pub(crate) fn new(text: &str)->Tokenizer{ //
        Tokenizer{
            pos: 0,
            s: text.parse().unwrap(),
            token: Token::DEFAULT
        }
    }

    pub(crate) fn helper(text: &str)->Tokenizer{
        let mut t = Tokenizer::new(text);
        t.token = Tokenizer::next(&t);
        return t;
    }

    pub fn next(&self) -> Token {
        if self.s.len() <= self.pos{
            return Token::EOS;
        }

        let my_vec: Vec<char> = self.s.chars().collect(); //Rust doesn't allow String indexing

        return match my_vec[self.pos] { //loop?
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
                Token::DEFAULT
            }  //rest of symbols
        }

    }

    pub(crate) fn next_token(&mut self){
        self.token = Tokenizer::next(&self);
    }

}

/*
pub fn scan(tok: Tokenizer) -> Vec<Token> {

    let mut v: Vec<Token> = vec![];
    let mut t: Token = Token::DEFAULT;

    loop {
        t = Tokenizer::next(&tok);
        v.push(t);

        match t {
            Token::EOS => {break;}
            _ => {}
        }
    }
    return v;
}


pub fn show(tok: Tokenizer) -> String{
    let mut v: Vec<Token> = Tokenizer::scan(tok);
    let mut s: String = "".to_string();

    for n in 0..v.count(){
        s = s + show_tok(v[n]);

        if n + 1 < v.size() {
            s = s + ";";
        }
    }
    return s;
}
*/
