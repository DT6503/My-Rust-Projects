
use std::cell::RefCell;


struct Logger{
    history: RefCell<Vec<String>>,
}

impl Logger{
    fn new()->Self{
Logger { history: RefCell::new(Vec::new()), }
    }
}

trait Plagin{
    fn log(&self, message:&str);
}


impl Plagin for Logger{
    fn log (&self, message:&str){
self.history.borrow_mut().push(String::from(message));
    }
}

struct PlugingManager{
    logger: Logger,
}

impl PlugingManager{
    fn run_plugins(&self){
        self.logger.log("Движок запущен");
        self.logger.log("Плагины инициализированы");
    }
}


fn main() {
    
let manager = PlugingManager{logger: Logger::new() };

manager.run_plugins();

let history_borrow = manager.logger.history.borrow();
println!("Записсей в логе: {:#?}", history_borrow);

}
