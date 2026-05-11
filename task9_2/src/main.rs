
fn work_with_string(str:&str)->Result<i32, std::num::ParseIntError>{
    let mut num = str.parse::<i32>()?;
//return
    Ok(num*2)

}

fn main() {
    let s = "10";
    match work_with_string(s){
        Ok(n)=>println!("Результат: {}", n),
        Err(e)=>println!("Ошибка: {}", e),
    }

    let str1= "abc";
if let Err(error) = work_with_string(str1){
    println!("Проверка на 'abc': Ошибка ({})", error);
}

}