
enum MenuItem{
    Pizza(String),
    Drink(String, f64),
    H(i32),
    n(i32),
}


fn main() {
    println!("Hello, world!");
    let mut cart:Vec<MenuItem> = Vec::new();
    cart.push(MenuItem::Drink(String::from("Cola"), 12.6));
cart.push(MenuItem::Pizza(String::from("Margaritta")));

cart.push(MenuItem::H(32));
cart.push(MenuItem::n(1));

for food in &cart{ //не изменяемое заимствование cart

match food{
    MenuItem::Pizza(name)=> println!("Пицца! {}", name),
    MenuItem::Drink(name,price ) => println!("Напитки: {} объёмом: {:.2}", name, price),
    _=>println!("Другие товары")
}
//ОШИБКА: тк изменяемое действие с cart
//cart.push(MenuItem::H(21));

};




}
