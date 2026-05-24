use std::fmt::format;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    //channel

    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();
    let tx3 = tx.clone();

    thread::spawn(move || {
        for i in 1..=3 {
            tx.send(format!("Модуль [Auth]: Действие №{} выполнено", i))
                .unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    thread::spawn(move || {
        for i in 1..=3 {
            tx2.send(format!("Модуль [Database]: Действие №{} выполнено", i))
                .unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    thread::spawn(move || {
        for i in 1..=3 {
            tx3.send(format!("Модуль [Payment]: Действие №{} выполнено", i))
                .unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    println!(" Центральный сборщик логов запущен ");

    for log_message in rx {
        println!("{}", log_message);
    }
    println!("Все модули завершили работу. Программа успешно закрыта")
}
