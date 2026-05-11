
fn count_len(s:&str) -> usize {s.len()}


fn main() {
    
let x = Box::new(String::from("Привет Даша"));
println!("{}", count_len(&x));

}
