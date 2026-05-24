use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            thread::sleep(Duration::from_millis(1500));
            tx.send(format!("Message №{}", i)).unwrap();
        }
    });

    println!("--- Система запущена. Главный поток начинает опрос воркера ---");

    loop {
        thread::sleep(Duration::from_millis(500));
        println!("--- Главный поток проверяет почту... ---");

        match rx.try_recv() {
            Ok(report) => {
                println!("Получен отчёт от воркера: {}", report);
            }
            Err(TryRecvError::Empty) => {
                println!("На почте пусто. Воркер всё еще трудится над задачей...");
            }

            Err(TryRecvError::Disconnected) => {
                println!("Связь потеряна: воркер закрыл канал. Завершаем опрос.");
                break;
            }
        }
    }
    println!("--- Все задачи обработаны. Главный поток успешно завершил работу! ---");
}
