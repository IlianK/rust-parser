pub enum Exp {
    Int {
        val: i32,
        b: bool
    },
    Plus {
        e1: Box<Exp>,
        e2: Box<Exp>,
        b: bool
    },
    Mult{
        e1: Box<Exp>,
        e2: Box<Exp>,
        b: bool
    },
}
impl Exp {
    pub fn set(&mut self, set_to: bool) -> (){
        return match self{
            Exp::Int {ref mut b, .. } => {
                *b = set_to
            }
            Exp::Plus {ref mut b, .. } => {
                *b = set_to
            }

            _ => {}
        }
    }

    pub fn eval(self: &Exp) -> i32 {
        return match self {
            Exp::Int { val, .. } => *val,
            Exp::Plus { e1, e2, .. } => {
                e1.eval() + e2.eval()
            },
            Exp::Mult { e1, e2, .. } => {
                e1.eval() * e2.eval()
            }
        }
    }

    pub fn pretty(self) -> String {
        return match self {
            Exp::Int { val, .. } => {
                val.to_string()
            },

            Exp::Plus { e1, e2, b } => {
                //println!("b: {}", b);
                if b { //if true Mult has been visited
                    let s = format!("( {} + {} )", e1.pretty(), e2.pretty());
                    s.to_string()
                }
                else { //if false Mult has not been visited
                    let s = format!("{} + {}", e1.pretty(), e2.pretty());
                    s.to_string()
                }
            },

            Exp::Mult { mut e1, mut e2, .. } => {
                e1.set(true);
                e2.set(true);

                let s = format!( "{} * {}", e1.pretty(), e2.pretty());
                s.to_string()
            }
        }
    }
}