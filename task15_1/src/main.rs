fn main() {

let x = 5;
let y = Box::from(x);

    println!("Stack: {}, Heap {}", x, y);
}
