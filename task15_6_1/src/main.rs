use std::rc::Rc;

fn main() {
    let strong1=Rc::new(String::from("Rust"));

    assert_eq!(Rc::strong_count(&strong1), 1);
    assert_eq!(Rc::weak_count(&strong1), 0);

let strong2 = Rc::clone(&strong1);

 assert_eq!(Rc::strong_count(&strong1), 2);
    assert_eq!(Rc::weak_count(&strong1), 0);


    let weak1= Rc::downgrade(&strong1);
assert_eq!(Rc::strong_count(&strong1), 2);
    assert_eq!(Rc::weak_count(&strong1), 1);

    
drop(strong2);
assert_eq!(Rc::strong_count(&strong1), 1);
    assert_eq!(Rc::weak_count(&strong1), 1);

}
