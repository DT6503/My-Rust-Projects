mod front_of_house{
    pub mod hosting{
       pub  fn add_to_waitlist(){}
    }
}

pub fn eat_at_restaurant(){
    //Absoulute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Realative path
    front_of_house::hosting::add_to_waitlist();
}


/////////  2  пример
fn serve_oder(){}

mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::serve_oder();
    }

    fn cook_order(){}

}