
use std::cell::RefCell;
use std::rc::Rc;

struct ChatRoom{
    messages: RefCell<Vec<String>>,
    
}

struct User{
    name:String,
    room:Rc<ChatRoom>,
}

impl ChatRoom{
    fn new()->Self{
        ChatRoom { 
            messages: RefCell::new(Vec::new()) 
        }
    }
}

impl User{
    fn join_room(name:&str, room:Rc<ChatRoom>) ->Self{
        User { name: name.to_string(), room }
    }

fn send_message(&self, text:&str){
    let formatted = format!("{}: {}", self.name, text);
    self.room.messages.borrow_mut().push(formatted);
}

}

fn main(){

let room = Rc::new(ChatRoom::new());
let user1 = User::join_room("Даша", Rc::clone(&room));
let user2 = User::join_room("Дмитрий", Rc::clone(&room));

user1.send_message("ПРивет");
user2.send_message("Привет! Как дела с Rust?");
user1.send_message("Стараемся, придерживаясь бразильской системы");

//без iter влад-ие переёдёт циклу
for msg in room.messages.borrow().iter(){
println!("{}", msg);
}

}