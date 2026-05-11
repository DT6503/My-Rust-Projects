
//Enum с данными

enum OrderStatus{
    Pending,
    Shipped (String),
    Delivered,
    Cancelled (String),
}

fn main() {
let a = OrderStatus::Shipped(String::from("12"));
print_status(a);
}

fn print_status(status: OrderStatus){
    match status{
OrderStatus::Delivered=> println!("Доставлено"),
        OrderStatus::Shipped(trac_number) => println!("Заказ отправлен, трек-номер: {}", trac_number),
        OrderStatus::Cancelled(reason) => println!("Заказ отменен по причине: {}", reason),
   OrderStatus::Pending=>println!("Ожидаем обработки"),
    }
}
