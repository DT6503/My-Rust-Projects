use std::collections::HashMap;

fn main() {
    
let mut hash = HashMap::new();
hash.insert("Green", 19);
hash.insert("Red", 12);
let a = hash.entry("Blue").or_insert(1); //изм-мая ссылка на знаение

println!("{}", a);
println!("{:?}", hash);
//println!("{}", a); //тут не работает

}
