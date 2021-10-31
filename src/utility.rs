use crate::ast::Exp;

struct Optional<T:Exp>{
    b: bool,
    val: T
}

impl Optional<Exp> {
    fn new()-> Optional<T> {
        Optional {b: false, val: () }
    }
    fn new_with_t(v: T) -> Optional<T> {
        Optional {b: true, val: v}
    }

    fn is_just(&self) -> bool{
        return self.b;
    }

    fn is_nothing(&self) -> bool{
        return !(self.b);
    }

    fn from_just(&self) -> T{
        return self.val;
    }
}

pub(crate) fn nothing() -> Optional<T> {
    return Optional::new();
}

pub(crate) fn just(v: Box<Exp>) -> Optional<T> {
    return Optional::new_with_t(v);
}





/*
// Simulating Overloading Constructors
enum MixedOptional<T>{
    b(bool),
    boolVal(bool,T)
}

trait IntoMixedOptional<T>{
    fn into(self) -> MixedOptional<T>;
}

impl MixedOptional<T>{
    fn new<A>(args:A) -> MixedOptional<T>
        where A: IntoMixedOptional<T>
    {
        args.into()
    }
}

//MixedOptional::new(bool);
impl IntoMixedOptional<T> for bool{
    fn into(self)-> MixedOptional<T> {
        MixedOptional<T>::b(self);
    }
}

//MixedOptional::new((bool,T));
impl IntoMixedOptional<T> for (bool, T){
    fn into(self)-> MixedOptional<T> {
        MixedOptional<T>::boolVal(self.0, self.1);
    }
}

trait Opt<T> {
    fn is_just(&self)->bool;
    fn is_nothing(&self)->bool;
    fn from_just(&self)->T;
}
*/



