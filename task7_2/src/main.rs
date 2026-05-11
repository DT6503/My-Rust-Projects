fn clean_table() {
    println!("Cleaning the table...");
}

mod kitchen {
    pub fn fix_order() {
        cook_order();
        // ТУТ ОШИБКА: как вызвать clean_table, которая находится СНАРУЖИ модуля kitchen?
        super::clean_table(); 
        
    }

    fn cook_order() {
        println!("Cooking...");
    }
}

fn main(){
    kitchen::fix_order();
}