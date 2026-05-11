


fn main() {
    let mut message = String::from("Привет!");
    let read = ||{
        println!("{}", message);
    };

    read();

    let mut change = ||{
        message.push_str("!");
    };


    change();
   
}
