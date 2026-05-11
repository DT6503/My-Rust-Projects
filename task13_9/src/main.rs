fn main() {
    let logs = vec!["200", "404", "error_99", "500", "not_a_number"];

let val : Vec<(usize, i32)>= logs
.iter()
.enumerate()
.filter_map(|(index, value)| {
match value.parse::<i32>() {
    Ok(num)=>Some((index, num)),
    Err(_)=>None,
}
})
.collect();


for (index, status) in val {
        println!("Запись №{}: статус {}", index, status);
    }

}
