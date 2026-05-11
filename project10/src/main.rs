use std::fmt::format;
#[derive(Debug)]
#[derive(Clone)]
struct LogEntry<'a>{
level: String,
message: &'a str,
}

trait Printable{
    fn format_log(&self)->String;
}


impl <'a> Printable for LogEntry<'a>{
    fn format_log(& self)->String {
       format!("[{}] {}", self.level, self.message)
    }
}
#[derive(Debug)]
struct LocalStorage<T>{
    items: Vec<T>,
}

impl <T: Printable + Clone> LocalStorage<T> {
    
    fn add(&mut self, item:T){
        self.items.push(item);
    }

fn new ()->Self{
    LocalStorage { items: Vec::new() }
}

fn print_all(&self){
    for item in &self.items{
        println!("{}", item.format_log());
    }
}

fn find_by_level(&self, level:&str) ->LocalStorage<T>{
let mut filtered_items = Vec::new();

for item in &self.items{
    if item.format_log().contains(level){
        filtered_items.push(item.clone());
    }
}
LocalStorage{items:filtered_items}
}

}


fn main() {
   let mut storage = LocalStorage::new();

    // Эти строки живут вечно ('static), поэтому storage будет доволен
    let msg1 = "Система запущена"; 
    let msg2 = "Ошибка доступа к базе данных";

    storage.add(LogEntry {
        level: String::from("INFO"),
        message: msg1,
    });

    storage.add(LogEntry {
        level: String::from("ERROR"),
        message: msg2,
    });

  storage.add(LogEntry {
        level: String::from("INFO"),
        message: "Третья",
    });

    storage.print_all();

    let stor1 = storage.find_by_level("INFO");
    println!("{:?}", stor1);
}
