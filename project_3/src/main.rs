fn main() {
    let mut x = 5;

    println!("The value of x, {}", x);
    x = 6;
     println!("The value of x, {}", x);

     const SUBSCRIBE_COUNT: u32 = 100_000; //1. нельзя никогда изм-ить даже через mut 2. без знаковое 3. не можем ей присвоить значение, которое высчитывается во время выполнения

     //затенение: создание новой пермен-ой с исп-ем уже существующего имени
     let y = 5;
     let y = "str";


//кортеж - массив связанный данных фиксированного размера, д-ые могут быть разных типов
let tup = ("Let,s get Rusty", 100_000);
let (channel, sub_count) = tup;
let sub_count = tup.1;


//массивы имею фикс длину
let error_codes = [200, 404, 500];
let not_found = error_codes[1];


let byte = [0;8]; //8 эл-ов и все они равны 0

let sum = my_function(11, 22);
println!("The sum: {}", sum);



}



//функция
fn my_function(x:i32, y:i32)->i32{
    println!("Another function! x = {}", x);
 println!("Another function! y = {:.2}", y);


 let sum = x+y;
sum // = return sum
}