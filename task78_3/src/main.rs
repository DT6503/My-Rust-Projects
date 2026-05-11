use std::collections::HashMap;

use std::fmt::Display;


fn main() {
    let orders_log = "pizza coffee pizza burger coffee pizza";
let mut hash = HashMap::new();

for i in orders_log.split_whitespace(){
    let count = hash.entry(i).or_insert(0); //ссылка на значения в хэше
    *count+=1; //ибо это ссылка
}

println!("{:?}", hash);

}
