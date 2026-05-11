
struct Storage<T>{
    item: T,

}

impl <T> Storage<T>{
    fn new (item: T) -> Self{
        Storage{item}
    }

    fn get_item(&self)->&T{
&self.item
    }
}


fn main() {
   
let int_storage = Storage::new(100);
let string_storage = Storage::new(String::from("Rust Data"));

println!("На первом складе: {}", int_storage.get_item());
println!("На втором складе: {}", string_storage.get_item());

}
