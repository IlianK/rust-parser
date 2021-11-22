use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


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
    /*
    PlusN{
        e: Vec<Exp>
    }
    */

    Mult{
        e1: Box<Exp>,
        e2: Box<Exp>,
        b: bool
    },
}
impl Exp {
    pub fn set(&mut self, set_to: bool) -> (){
        return match self{
            Exp::Int { val,  ref mut b} => {
                //println!("int");
                *b = set_to
            }
            Exp::Plus { e1, e2, ref mut b} => {
                //println!("plus");
                *b = set_to
            }
            Exp::Mult { e1, e2, mut b} => {
                //println!("mult");
                //b = true
            }
        }
    }

    pub fn eval(self: &Exp) -> i32 {
        return match self {
            Exp::Int { val, b } => *val,
            Exp::Plus { e1, e2, .. } => {
                e1.eval() + e2.eval()
            },
            Exp::Mult { e1, e2, .. } => {
                e1.eval() * e2.eval()
            }

            /*
            Exp::PlusN { e } => {
                let expressions = e;
                let mut i: i32 = 0;
                let x = expressions[i];
                /*loop{
                    if type_of(expressions[i]) == type_of(i){
                    }
                    i += 1;
                }
                */
                i
             */

        }
    }

    pub fn pretty(mut self) -> String {
        return match self {
            Exp::Int { val, mut b } => {
                val.to_string()
            },

            Exp::Plus { e1, e2, mut b } => {
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

            Exp::Mult { mut e1, mut e2, mut b } => {
                e1.set(true);
                e2.set(true);

                let s = format!( "{} * {}", e1.pretty(), e2.pretty());
                s.to_string()
            }
            /*
            Exp::PlusN { e } => {
                "".to_string()
            }
             */
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