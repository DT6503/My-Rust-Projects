fn main() {
    let mut s = String::from("Daria and Daria want to learn Rust");

let c = s.chars().nth(2);
println!("{:?}", c);

let s = String::from("Я люблю Java");
let new_s = s.replace("Java", "Rust"); // "Я люблю Rust"
println!("{}", new_s);
println!("{}", s);

}
