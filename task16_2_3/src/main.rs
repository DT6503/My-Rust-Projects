use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    //фоновый
    let (tx_to, rx_to) = mpsc::channel();
    //главный
    let (tx_back, rx_back) = mpsc::channel();

    //фоновый п-к
    thread::spawn(move || {
        while let Ok(message) = rx_to.recv() {
            if message == "Стоп" {
                println!("[Поток]: Получена команда 'Стоп'. Завершаю работу...");
                break;
            }

            if message == "Пинг" {
                println!("[Поток]: Получил '{}' -> Отправляю 'Понг'", message);
                thread::sleep(Duration::from_millis(300));
                tx_back.send(String::from("Понг")).unwrap();
            }
        }
    });

    //main п-к
    for i in 1..=4 {
        println!("Раунд №{}", i);

        println!("[Main]: Отправляю 'Пинг'...");
        tx_to.send(String::from("Пинг"));

        let answer = rx_back.recv().unwrap();
        println!("[Main]: Получил ответ: '{}'", answer);

        thread::sleep(Duration::from_millis(500));
    }

    println!("[Main]: Матч окончен. Отправляю 'Стоп'..."); //не хотим битые файлы, поэтому нужен стоп
    tx_to.send(String::from("Стоп")).unwrap();

    thread::sleep(Duration::from_millis(100));
    println!("--- Программа успешно завершена! ---");
}
