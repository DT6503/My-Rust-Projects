//use of moved value

use std::thread;
use std::time::Duration;

fn main() {
    let data = String::from("Секретные данные");

    let data1 = 2;

    let handle = thread::spawn(move ||{
        println!("Поток обрабатывает: {}", data);
        thread::sleep(Duration::from_millis(10));
    });

    //println!("Main: {}", data);
    handle.join().unwrap();
}
