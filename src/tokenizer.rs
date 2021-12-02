
pub enum Token{
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

pub struct Tokenizer{
    pos: usize,
    s: String,
    pub token: Token
}


impl Tokenizer {

    pub fn helper(text: &str)->Tokenizer{
        let mut t = Tokenizer{
            pos: 0,
            s: text.to_string(),
            token: Token::DEFAULT
        };
        t.token = Tokenizer::next(&mut t);
        return t;
    }


    pub fn next(&mut self) -> Token {
        if self.s.len() <= self.pos {
            return Token::EOS;
        }
        let my_vec: Vec<char> = self.s.chars().collect(); //Rust doesn't allow String indexing // self verbraucht

        loop {
            match my_vec[self.pos] {
                '0' => {
                    self.pos += 1; //self.pos = self.pos + 1
                    return Token::ZERO
                },
                '1' => {
                    self.pos += 1;
                    return Token::ONE
                },
                '2' => {
                    self.pos += 1;
                    return Token::TWO
                },
                '(' => {
                    self.pos += 1;
                    return Token::OPEN
                },
                ')' => {
                    self.pos += 1;
                    return Token::CLOSE
                },
                '+' => {
                    self.pos += 1;
                    return Token::PLUS
                },
                '*' => {
                    self.pos += 1;
                    return Token::MULT
                },
                __ => {
                    self.pos += 1;
                }  //rest of symbols
            }
        }
    }


    pub fn next_token(&mut self){
        self.token = Tokenizer::next(self);
    }

}


