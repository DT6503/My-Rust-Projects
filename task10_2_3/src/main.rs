use std::ops::Add;


use std::fmt::Display;
#[derive(Debug)]
struct Preview <'a> {
    first: &'a str,
}

fn main() {
    
let mut str = String::from("BRRRRRRRbrrrrrrrrrrrrrrhhhhhhaaaaaaa");

//drop(str); //moved

//mooved
//let str2 = String::from("ANALYSE");
//str.add(&str2);

let prev = Preview{first: &str[0..10]};

//Тут mut
//str.replace_range(0..1, "H");




println!("{:?}", prev);


}
