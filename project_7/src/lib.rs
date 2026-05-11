mod front_of_house { //доступен, потому что он 
    //находится в том же модуле, что и функция 
    //eat_at_restaurant

   pub mod hosting {
     pub   fn add_to_waitlist() {}

    }

}

pub fn eat_at_restaurant(){
    //Absoulute path
    crate::front_of_house::hosting::add_to_waitlist();

//Relative path
front_of_house::hosting::add_to_waitlist();
}


//2

fn serve_order(){}

mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }
fn cook_order(){}

}

