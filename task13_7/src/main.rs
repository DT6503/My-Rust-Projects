fn main() {
    let v1 = vec![1,2,3];

    for val in &v1{
        print!("{} ", val);
    }

    println!("");
////////////////////////
    let v1_iter = v1.iter();
    for val in v1_iter {      // используем итератор
    println!("Got: {val}");
    }


    //ПЕРВЫЙ ВАРИК
let v2 = vec![" ivan ", "Alice", " ", "BOB ", " charlie "];

let mut cleaned = Vec::new();

let v2_iter = v2.iter();
for val in v2_iter{
    let trimmed = val.trim();
    if !trimmed.is_empty() {
        cleaned.push(trimmed.to_lowercase());
}
}

println!("{:?}", cleaned);




//ВТОРОЙ ВАРИК
let v3 = vec![" ivan ", "Alice", " ", "BOB ", " charlie "];
let mut clean: Vec<String> = v2
.iter()
.map(|s| s.trim())
.filter(|s| !s.is_empty())
.map(|s| s.to_lowercase())
.collect();

println!("{:?}", clean);


}
