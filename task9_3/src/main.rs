
struct Username{
    name:String,
}

impl Username{
    fn new(name:String)->Result<Username, String>{
        if name.is_empty() || name.chars().count()>10{
            Err(String::from("Ошибка!"))
        }

        else{
            Ok(Username { name })
        }
    }
}


fn main() {
   let input = String::from("");
   match Username::new(input){
    Ok(user)=> println!("Успех! Имя: {}", user.name),
    Err(e)=>println!("Ошибка валидации: {}", e),
   }
}
