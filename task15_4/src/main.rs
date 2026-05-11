#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // Твоя задача: создай переменную list, которая выглядит как:
    // 1 -> 2 -> 3 -> Nil
    // Подсказка: начинай с самого конца (с Nil) или вкладывай друг в друга:
    // Cons(1, Box::new(Cons(2, ...)))
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:#?}", list);

let x = String::from("Привет");
let y = Box::new(x);
//let b = Box::new(x);

//println!("{}, {}", y, b);

}
