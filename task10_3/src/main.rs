use std::fmt::Debug; 

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T: Debug, U: Debug> Point<T, U> {
    fn print_info(&self) {
        println!("Первое знач: {:?} Второе знач: {:?}", self.x, self.y);
    }
}

impl Point<f32, f32>{
    fn is_positive(&self)->bool{
        self.x > 0.0 && self.y >0.0
}
}

fn main() {
    let p = Point { x: 5, y: "Daria" };
    p.print_info();

let p2:Point<f32,f32> = Point{x:3.2, y:4.6};
p2.print_info();
println!("{}",p2.is_positive());

}