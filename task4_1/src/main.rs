fn main() {
  
let x1 = 10;
let s1 = String::from("Rust");

let x2 = x1;
let s2 = s1;

println!("{}", x1);
println!("{}", x2);
//println!("{}", s1); //Нельзя, тк выполнилось перемещение
println!("{}", s2);

println!();

let s3 = s2.clone(); //глубокое копирование
println!("{}", s3);
println!("{}", s2);
}
