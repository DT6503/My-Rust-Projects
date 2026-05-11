use std::collections::HashMap;



fn main() {
    let text = "apple banana apple";
let mut map = HashMap::new();

for word in text.split_whitespace(){

let count = map.entry(word).or_insert(0); //изменяема ссылка на значение в мапке!! в прошлой задаче эсперементировала
*count += 1;

}

println!("{:?}", map);

}
