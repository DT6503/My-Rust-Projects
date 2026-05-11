fn main() {
    let mut x = Box::from(10);
    *x = 30;
println!("{}", *x);

}
