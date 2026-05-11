 pub mod hosting{
       pub fn add_to_waitlist() {
    println!("Добавлен в лист ожидания!");
}
    }


    pub use hosting::add_to_waitlist;//реэкспорт наверх