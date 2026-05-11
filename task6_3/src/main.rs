//if let

enum Message{
    Quit, 
    Move {x:i32, y:i32},  //структура 
    Write(String), //кортеж
    ChangeColor(i32, i32, i32),
}



fn main() {
    let msg = Message::Write(String::from("Учим Rust!"));

if let Message::Write(msg) = msg{
    println!("The text of meassage: {}", msg);
}


let x: i32 = 10;
let y: Option<i32> = Some(5);
let sum = x + y.unwrap_or(0); 
println!("The sum: {}", sum);

}
