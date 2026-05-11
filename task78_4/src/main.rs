use std::fmt::format;

fn prepare_bill(client: &str, total: f64) -> String{
    
    let result = format!("Client: {client}, Total: {total}");
    result

}


fn main() {
    let name = String::from("Daria");
    let money:f64 = 23.4;

println!("{}", prepare_bill(&name, money));

}
