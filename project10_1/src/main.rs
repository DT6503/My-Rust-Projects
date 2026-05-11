use std::fmt::Debug;
use std::fmt::Display;




#[derive(Debug)]
#[derive(Clone)]
struct LogEntry <'a>{
    level:String,
    message:&'a str,
}

trait Printable {
    fn format_log(&self)->String;
       
}


impl <'a> Printable for LogEntry<'a>{
    fn format_log(&self)->String {
        format!("[{}], {}", self.level, self.message)
    }
}

#[derive(Debug)]
struct LocalStorage <T> {
items: Vec<T>,
}

impl<T:Clone + Printable> LocalStorage<T>{
    fn add(&mut self, item:T){
self.items.push(item);
    }

fn new()->Self{
    LocalStorage { items: Vec::new() }
}

fn print_all(&self){
    for item in &self.items{
        println!("{:#?}", item.format_log());
    }
}

fn find_by_level(&self, level:&str) ->LocalStorage<T>{
    let mut filtered_items = Vec::new();
 for item in &self.items{
    if item.format_log().contains(level){
        filtered_items.push(item.clone());
    }
 }   
 LocalStorage { items: filtered_items }
}
}



fn main() {
   let mut storage = LocalStorage::new();

    let entry1 = LogEntry {
        level: String::from("INFO"),
        message: "Система запущена",
    };

    let entry2 = LogEntry {
        level: String::from("ERROR"),
        message: "Ошибка доступа к базе данных",
    };

    let entry3 = LogEntry {
        level: String::from("INFO"),
        message: "Пользователь вошёл в систему",
    };

    storage.add(entry1);
    storage.add(entry2);
    storage.add(entry3);

    println!("=== Все логи ===");
    storage.print_all();

    println!("\n=== Логи с уровнем INFO ===");
    let info_logs = storage.find_by_level("INFO");
    info_logs.print_all();

    println!("\n=== Логи с уровнем ERROR ===");
    let error_logs = storage.find_by_level("ERROR");
    error_logs.print_all();
}
