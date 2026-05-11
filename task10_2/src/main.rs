use std::fmt::Display;
use std::fmt::Debug;

struct Point<T, U>{
    x:T,
    y:U,
}

impl<T:Debug, U:Debug> Point<T,U>{
    fn print_info(&self)
    {
        println!("Первое знач: {:?} Второе знач: {:?}", self.x, self.y);
    }
}


fn main() {
  let p = Point{x:5, y:"Daria"};
  let p1 = Point{x:5, y:8};

  p.print_info();
  p1.print_info();
}
