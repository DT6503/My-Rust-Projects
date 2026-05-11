use core::num;


fn divide (a:f64, b:f64)->Result<f64, String>{
    
       if b==0.0{
Err(String::from("Division by zero"))
       }
       else{
        Ok(a/b)
       }
    



}

fn main() {

let numbers = [(10.0, 2.0), (3.0, 0.0)];

//просто выводим в консоль
for (a,b) in numbers{
    match divide(a, b) {
Ok(result)=>println!("{a} / {b} = {result}"),
Err(e)=>println!("Ошибка при делении {a} на {b}: {e}"),
    }
}

}