
use std::fmt::Display;
#[derive(Debug)]
struct Wrapper<T>{
val:T,
}

impl<T> Wrapper<T>{
fn replace_and_return_old<U>(self,new_val:U)->(U,T){
(new_val, self.val)

}
}


fn main() {
    let w1=Wrapper{val:1};
    println!("{:?}", w1);
    println!("Turns into {:?}", w1.replace_and_return_old(5));

 let w2=Wrapper{val:"Daria"};
    println!("{:?}", w2);
    println!("Turns into {:?}", w2.replace_and_return_old(5));

}
