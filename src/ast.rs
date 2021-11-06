
pub trait Exp{                //base trait for each expression type
    fn eval(&self)->i32;
    fn pretty(&self)->String;   //mut self so struct parameters can be changed over time not necessary for read functions
}

pub struct Int{
    pub(crate) val: i32,
}

/*
pub struct PlusN<T:Exp> {
    pub(crate) operands : Vec<T>
}
*/

pub struct Plus<T:Exp>{
    pub e1: T,
    pub e2: T,
}


pub struct Mult<T:Exp>{
    pub e1: T,
    pub e2: T,
}

impl Exp for Box<dyn Exp>{

    fn eval(&self) -> i32 {
        let x = 0;
        return x;

    }

    fn pretty(&self) -> String {
        let x = "here?".to_string();
        return x;
    }

}


impl Exp for Int {   //implementing trait for IntExp (overloading methods)

    fn eval(&self) -> i32 {
        return self.val
    }

    fn pretty(&self)->String{
        return self.val.to_string();
    }
}


impl<T:Exp> Exp for Plus<T>{
    fn eval(&self) -> i32 {
        return self.e1.eval() + self.e2.eval();
    }

    fn pretty(&self) -> String {
        let mut s = "or there?";
        /*
        if self.been_there {
            let s = format!( "( {} + {} )", self.e1.pretty(), self.e2.pretty());
        }
        else {
            let s = format!( "{} + {} ", self.e1.pretty(), self.e2.pretty());
        }
        */
        return s.parse().unwrap();
    }
}


impl<T:Exp> Exp for Mult<T> {

    fn eval(&self) -> i32 {
        return self.e1.eval() * self.e2.eval()
    }

    fn pretty(&self)->String{
        //self.e1.set_been_there(true);
        //self.e2.set_been_there(true);

        let s = format!( "( {} * {} )", self.e1.pretty(), self.e2.pretty());

        return s.parse().unwrap();
    }
}



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
