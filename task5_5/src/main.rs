#[derive(Debug)]
struct Rectangle{
length:i32,
width:i32,
}

fn main() {
    println!("Hello, world!");
}

fn can_hold(&self, other: &Rectangle)->bool{
self.length > other.length && self.width > other.width
}
