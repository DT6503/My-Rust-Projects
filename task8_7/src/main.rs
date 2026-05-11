fn main() {
    use std::collections::HashMap;
let mut map = HashMap::new();
let key = String::from("Name");
let value = String::from("Daria");

map.insert(key.clone(), value.clone());

println!("{}, {}", key, value);

}
