fn main() {
    println!("Hello, world!");
}

pub trait Dot<RHS> {
    type Output;
    fn dot(&self, rhs: &RHS) ->  Self::Output; 
}

struct CrypticType<T> { val: T }

pub trait Add<RHS,Result> {
    fn add(&self, rhs: &RHS) -> Result;
}


impl Add<RHS, U> for CrypticType<T> 
     add(&self, rsh: U) -> U {
         &self.val + rsh.val
    }
}
 
 




