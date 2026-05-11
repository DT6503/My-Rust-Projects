fn main() {
   
   let besk: Vec<i32> = (1..)
   .skip(10)
   .filter(|x| x%3 ==0)
   .take(5)
   .map(|x|x*x)
   .collect();

   
    println!("{:?}", besk);
}
