

fn main() {
  let transactions = vec![100, -20, 50, -10, 200, -150];

  let summa:i32 = transactions
  .iter()
  .filter(|&&s| s>0)
  .sum();
  
  println!("Твой доход: {}", summa);

let raz:i32 = transactions
.iter()
.filter(|&&s| s< 0)
.sum();

println!("Твои расходы: {}", raz);

    println!("Hello, world!");
}
