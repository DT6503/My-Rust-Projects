/*fn main() {
    a();
}


fn a (){
        b();
    }

    fn b(){
        c(21);
    }

    fn c(num:i32){
if num == 22
{
    panic!("Don't do it with 22!!!");
} 
    }*/


    use std::fs::File;
    use std::io::ErrorKind;//позволит получить тип ошибки

fn main() {
    let f = File::open("hello.txt");
    // тип: Result<std::fs::File, std::io::Error>

let f = match f {

    Ok(file)=>file,
    //Err(error)=>panic!("Prpblem opening the file: {:?}", error),
    Err(error)=> match error.kind(){
        ErrorKind::NotFound=>match File::create("hello.txt"){
            Ok(fc)=>fc,
            Err(e)=>panic!("Problem creating the file: {:?}", e),
        },
      
    other_error=>panic!("Problem opening the file: {:?}", other_error) //содержит исходную прееменную
    }

};


}