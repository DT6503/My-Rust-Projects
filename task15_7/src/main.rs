
use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 1. Сначала реализуем обычный Deref (чтение)
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 2. Теперь реализуем DerefMut (изменение)
impl<T> DerefMut for MyBox<T> {

    /*"С этого момента моя структура официально умеет разыменовываться. 
    Когда кто-то использует * или вызывает методы данных внутри,
     используй логику, которую я напишу ниже". */
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0 // Возвращаем ИЗМЕНЯЕМУЮ ссылку
    }
}

fn main() {
    // Помечаем сам MyBox как mut
    let mut x = MyBox::new(5);

    // Благодаря DerefMut это теперь работает:
    *x = 10; 

    println!("Результат: {}", *x);
}