
//Анализатор текста "Memory Guard"


fn main() {

    //1
    process_text(String::from("I'm Daria!!!"));

let first = String::from("I'm Dima!!!");
let second = first.clone();

//println!("The second word!!!: {}", first.clone());

process_text(first);
//println!("{}", first); //владение уже у ф-ции

println!("The second word: {}", second);


//2
println!("{:#?}", get_status(second));

//3
let threshold: i32 = 12;
check_threshold(threshold);
println!("The value from main.rs: {}", threshold);

}


//1
fn process_text(str:String)
{
    println!("Our string is: {}", str);
}

//2
fn get_status(str:String)->(String, usize)
{
let len = str.len();
(str, len)
}


//3
fn check_threshold(num:i32){
println!("The value from function: {}", num);
}