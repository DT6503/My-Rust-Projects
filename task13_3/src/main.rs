fn main() {
    
    let data = String::from("Secret");
    
    let x_print  = ||{
        println!("{}", data);
    };

x_print();

    let x_movable = move ||{
        drop(data);
    };  


x_movable();

//x_print();



}
