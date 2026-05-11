use task11_1::is_strong_password;
fn main() {
    println!("Тестирование функции проверки пароля");
    
    let test1 = "Daia!";
    println!("Пароль '{}' сильный? {}", test1, is_strong_password(test1));
    
    let test2 = "DariaIs!Strong";
    println!("Пароль '{}' сильный? {}", test2, is_strong_password(test2));
}
