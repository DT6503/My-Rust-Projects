use task7_1::Food;
use task7_1::serve_order;

fn main() {
    let pizza = Food::new(String::from("Маргарита"), 450.0);

println!("Блюдо: {}", pizza.name);
    println!("Цена: {:.2} руб.", pizza.price);

println!("\n=== Процесс заказа ===");
serve_order();

}
