use std::collections::HashMap;

fn main() {
   let mut map = HashMap::new();
let key = String::from("Status");
map.insert(&key, 10);
println!("{}", key); // Ошибка здесь! тк произошло перемещение

}
