


/*
no rust-constructor: objects created automatically by passing
parameters directly: Exp:  new(MultExp{e1: a, e2: b});
*/



pub trait Exp{                //base trait for each expression type
fn eval(&self)->i32;
    fn pretty(&self)->String;   //mut self so struct parameters can be changed over time not necessary for read functions

    fn set_been_there(&self, been_there: bool){
        self.been_there = been_there;
    }
}

pub struct Int<T:Exp>{
    pub(crate) been_there: bool,
    pub(crate) val: i32
}

pub struct PlusN<T:Exp> {
    pub(crate) operands : Vec<T>
}


pub struct Plus<T:Exp>{
    pub(crate) been_there: bool,
    pub(crate) e1: T,
    pub(crate) e2: T
}


pub struct Mult<T:Exp>{
    pub(crate) been_there: bool,
    pub(crate) e1: T,
    pub(crate) e2: T
}


impl<T:Exp> Exp for Int<T> {   //implementing trait for IntExp (overloading methods)

    fn eval(&self) -> i32 {
        return self.val
    }

    fn pretty(&self)->String{
        return to_string(self.val);
    }
}




impl<T:Exp> Exp for PlusN<T> {

    fn eval(&self) -> i32 {
        return self.operands.iter().sum();
    }

    fn pretty(&self)->String{

        let s = "";

        for i in self.operands{

            if i == self.operands.last() {
                s.append(i);
                break;
            }

            s.append(i + "+");
        }
        return s.parse().unwrap();
    }
}


impl<T:Exp> Exp for Mult<T> {

    fn eval(&self) -> i32 {
        return self.e1.eval() * self.e2.eval()
    }

    fn pretty(&self)->String{
        self.e1.set_been_there(true);
        self.e2.set_been_there(true);

        let s = "";
        s.append(self.e1.pretty());
        s.append("*");
        s.append(self.e2.pretty());
        s.append("");

        return s.parse().unwrap();
    }
}