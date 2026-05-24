use std::rc::{Rc, Weak};


struct Parent{
    id:u32,
}

struct Child{
    parent:Weak<Parent>,
}


impl Child {
    fn print_parent_info(&self){
match self.parent.upgrade(){
Some(parent)=>println!("The id:{}", parent.id),
None => println!("Родитель был удалён")
}
    }
}


fn main(){
    let child = {
        let parent = Rc::new(Parent{id: 42});
        let c = Child {
            parent: Rc::downgrade(&parent),
    };
    println!("--- Внутри области видимости родителя ---");
        c.print_parent_info(); // Родитель жив, ID: 42
c
};

println!("--- Вне области видимости родителя ---");
    child.print_parent_info(); // Родитель был удален


}

