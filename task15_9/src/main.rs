use std::rc::Rc;
struct Owner {
    name: String,
}

struct Gadget {
    name: String,
    owner: Rc<Owner>, 
}

fn main() {
    let owner = Rc::new(Owner { name: String::from("Даша") });
    
    // Ошибка: тут владение owner уходит в телефон
    let phone = Gadget { name: String::from("iPhone"), owner:Rc::clone(&owner) }; 
    
    // Ошибка компиляции: owner уже перемещен, мы не можем отдать его ноутбуку!
    let laptop = Gadget { name: String::from("MacBook"), owner:Rc::clone(&owner) }; 
    
    println!("Гаджет {} принадлежит {}", phone.name, phone.owner.name);
    println!("Гаджет {} принадлежит {}", laptop.name, laptop.owner.name);
}
