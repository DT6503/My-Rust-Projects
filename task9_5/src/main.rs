use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {  // Box<dyn Error> — означает "любой тип ошибки"
    let f = File::open("config.txt")?; 
    //ошибок не было → возвращаем Ok(())
    Ok(())
}
