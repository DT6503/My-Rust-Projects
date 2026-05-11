use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Правильный способ: реализуем трейт
impl<T> Deref for MyBox<T> {
    type Target = T; // Вот здесь мы говорим, что "цель" — это тип T

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = MyBox::new(3);
    
    // 1. Теперь это работает через явный вызов
    let y = *(x.deref()); 
    
    // 2. И теперь "магия" включена: можно просто использовать *
    let z = *x; 
    
    println!("y = {}, z = {}", y, z);
}