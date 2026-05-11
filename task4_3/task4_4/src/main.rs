
// Система управления инвентарем (Ownership & References)

fn main() {
    
let mut inventory = String::from("Apples, Oranges");

show_inventory(&inventory);

let tomatato:&str = ", Tomato";
add_item(&mut inventory, &tomatato);

let s1 = &mut inventory;
//let s2 = &mut inventory;
add_item(s1, &tomatato);//тк время жизни ссылки заканчивается там, где она была последний раз использована

println!("{}", inventory);


let r1 = &inventory;
//add_item(&mut inventory, ", Grapes");
println!("!!! {}", r1);

/*Тот, кто держит ссылку r1, ожидает, что данные 
под ней не изменятся, пока он ими пользуется. 
Если мы позволим add_item изменить строку, данные 
в r1 могут стать невалидными (например, строка в 
куче переедет в другое место из-за нехватки места 
для новых символов, и старый указатель в r1 будет 
смотреть в пустоту).*/


}



fn show_inventory(data: &String){
println!("Data from function: {}", data);
}


fn add_item(data: &mut String, item: &str)
{
data.push_str(item);
}