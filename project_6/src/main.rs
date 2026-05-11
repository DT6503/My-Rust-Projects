enum IpAddrKind{
    V4(String),
    V6(String),
}

struct IpAddr{
    kind:IpAddrKind,
    address:String,
}

enum Message{
    Quit,
    Move{x: i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}


impl Message{
    fn some_function(){
        println!("Let's get Rusty!");
    }
}

fn main() {
   let four = IpAddrKind::V4;
   let six = IpAddrKind::V6;

   //дублирование информации! Адрес хранится дважды:
let localhost = IpAddr{
    kind:IpAddrKind::V4(String::from("127.0.0.1")),
    address: String::from("127.0.0.1"),
};

//правильно
let localhost1 = IpAddrKind::V4(String::from("127.0.0.1"));



//Option (Не обязательные переменные)
let some_number = Some(5);
let absenr_number:Option<i8> = None;

let x:i8=5;
let y:Option<i8>=Some(5);//может быть целым числом, а может и не быть
//let sum = x+y; //error
let sum = x+y.unwrap_or(0);


//выражения соответвия
value_in_cents(Coin::Quarter(UsState::Alaska));


//match expr
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
println!("Five:{:?}", six);
println!("None:{:?}", none);



//if let syntax
let some_value = Some(3);
match some_value{
    Some(3) => println!("three"),
    _=>(),
}
//замена обычного соспоставления выше на if let syntax
if let Some(3) = some_value{
    println!("three");
}


}

fn route(ip_kind: IpAddrKind){}



//выражения соответвия

#[derive(Debug)]  // Позволяет выводить enum в println! и для отладки
enum UsState {
    Alabama,      // Алабама
    Alaska,       // Аляска
    Arizona,      // Аризона
    Arkansas,     // Арканзас
    California,   // Калифорния
}


enum Coin {
    Penny,    // 1 цент
    Nickel,   // 5 центов
    Dime,     // 10 центов
    Quarter(UsState),  // 25 центов
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {              
        Coin::Penny => {println!("Lucky penny!"); 1},     
        Coin::Nickel => 5,    
        Coin::Dime => 10,     
        Coin::Quarter(state) => {println!("State quater from {:?}!", state); 25},  
    }  // ← не нужна точка с запятой, так как это выражение
}


//match expr
fn plus_one(x: Option<i32>) ->Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i+1),
    }
}

