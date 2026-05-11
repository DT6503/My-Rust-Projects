//Безопасное деленеие

fn main() {
    
let result1 = div (2.0, 0.0);

match result1{
    Some(res) => println!("Результат: {}", res),
    None => println!("Ошибка: деление на ноль!"),
}

}


fn div (a:f64, b:f64)->Option<f64>{
    if b == 0.0{
        None
    } else {
        Some(a/b)
    }
}