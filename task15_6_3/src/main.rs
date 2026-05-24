
use std::rc::{Rc,Weak};
use std::cell::RefCell;

struct Owner{
    name:String,
    gadget:RefCell<Option<Rc<Gadjet>>>,
}


struct Gadjet{
    model:String,
    owner:Weak<Owner>,
}

impl Owner{
    fn buy_gadjet(owner_rc:&Rc<Self>, model:&str){
        //созд гаджет, для него нужна слабая ссылка, которая получается из Rc поэтому исп owner_rc:&Rc<Self>
        let gadjet = Rc::new(Gadjet{
            model: model.to_string(),
            owner: Rc::downgrade(owner_rc),
        });

        owner_rc.gadget.borrow_mut().replace(gadjet);
    }
}


impl Drop for Owner{
    fn drop(&mut self){
        println!("Owner {} удален из памяти!", self.name);
    }
}


impl Drop for Gadjet{
    fn drop(&mut self){
        println!("Gadget {} удален из памяти!", self.model);
    }
}



fn main() {
    println!("Начало");

{
    let owner = Rc::new(Owner{
        name: String::from("Даша"),
        gadget: RefCell::new(None),
    });

Owner::buy_gadjet(&owner, "Sumsung");

println!("Выход из обл вид");

}

println!("Конец");


}
