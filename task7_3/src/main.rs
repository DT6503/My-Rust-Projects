use task7_3::front_of_house::hosting; //для hosting::... в main(){}
use task7_3::front_of_house::hosting::add_to_waitlist;


//с реэкспортом
//use task7_3::front_of_house::add_to_waitlist;//пропустили hosting, тк реэкспорт


fn main() {
hosting::add_to_waitlist();
}
