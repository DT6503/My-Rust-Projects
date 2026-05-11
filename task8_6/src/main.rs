use std::fmt::format;

fn main() {
    let str = String::from("User");
    let id = 42;
    let str1 = String::from("@active");

let mut ready = String::new();
ready = str;
ready.push_str(&id.to_string());
ready.push_str(&str1);

println!("{}", ready);

//print!("{}", str);


//2
 let stra = String::from("User");
    let ida = 42;
    let str1a = String::from("@active");

let mut readya = String::new();

readya= format!("{}{}{}", stra, ida, str1a);

println!("{}", readya);
}
