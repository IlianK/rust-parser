pub enum Exp {
    Int {
        val: i32,
        been_there: bool
    },
    Plus {
        e1: Box<Exp>,
        e2: Box<Exp>,
        been_there: bool
    },
    Mult{
        e1: Box<Exp>,
        e2: Box<Exp>,
        been_there: bool
    },
}
impl Exp {

    pub(crate) fn set_bt(self: & mut Exp, bool_to_set: bool){
        return match self {
            Exp::Int { mut been_there, .. } => { been_there = bool_to_set },
            Exp::Plus {mut been_there, ..  } => { been_there = bool_to_set},
            Exp::Mult {mut been_there, .. } => { been_there = bool_to_set}
        }
    }

    pub(crate) fn eval(self: &Exp) -> i32 {
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

    pub(crate) fn pretty(self: Exp) -> String {
        return match self {
            Exp::Int { val, .. } =>  val.to_string(),

            Exp::Plus { e1, e2, been_there } => {
                return if been_there {
                    let s = format!("( {} + {} )", e1.pretty(), e2.pretty());
                    s.to_string()

                } else {
                    let s = format!("{} + {} ", e1.pretty(), e2.pretty());
                    s.to_string()
                }
            },

            Exp::Mult { mut e1, mut e2, .. } => {
                e1.set_bt(true);
                e2.set_bt(true);

                let s = format!( "( {} * {} )", e1.pretty(), e2.pretty());

                s.parse().unwrap()
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