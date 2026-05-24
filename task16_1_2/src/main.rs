
use std::thread;


fn main() {
    let job_id = 1;

    
    let handle1= thread::spawn(move|| {
    println!("Начинаем обработку задачи №{}", job_id);

    if job_id==13 {
        panic!("Обнаружено несчастливое число!");
    }
format!("Задача №{} успешно выполнена!", job_id)

});

match handle1.join(){
   Ok(msg) => println!("Успех: {}", msg),
        Err(_) => println!("Фоновый поток завершился с ошибкой (паникой)!"),
    }

}
