pub enum Exp {
    Int {
        val: i32,
    },
    Plus {
        e1: Box<Exp>,
        e2: Box<Exp>,
    },
    Mult{
        e1: Box<Exp>,
        e2: Box<Exp>,
    },
}
impl Exp {

    pub fn eval(self: &Exp) -> i32 {
        return match self {
            Exp::Int { val } => *val,
            Exp::Plus { e1, e2 } => {
                e1.eval() + e2.eval()
            },
            Exp::Mult { e1, e2} => {
                e1.eval() * e2.eval()
            }
        }
    }

    pub fn pretty(self: Exp) -> String {
        return match self {
            Exp::Int { val} =>  val.to_string(),

            Exp::Plus { e1, e2 } => {
                let s = format!("{} + {}", e1.pretty(), e2.pretty());
                s.to_string()
            },

            Exp::Mult { mut e1, mut e2} => {
                let s = format!( "{} * {}", e1.pretty(), e2.pretty());
                s.to_string()
            }
        }
    }
}


/*
pub struct PlusN<T:Exp> {
    pub(crate) operands : Vec<T>
}
*/
/*
impl<T:Exp> Exp for PlusN<T> {
    fn eval(&self) -> i32 {
        return 0 //self.operands.iter().sum();
    }
    fn pretty(&self)->String{
        let mut s = "";
        for i in self.operands{
            //if i == self.operands.last(){
                //s.append(i);
            //    break;
            //}
            //s.append(i + "+");
        }
        return s.parse().unwrap();
    }
}
*/