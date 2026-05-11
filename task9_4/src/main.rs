use std::collections::HashMap;


fn main() {
    let mut a = HashMap::new();
    a.insert(String::from("Alina"), 1);
    a.insert(String::from("Daria"), 12);

    get_daria_score(&a);

    // Принимаем знач в перем-ю
    let result = get_daria_score(&a);
match result {
        Some(val) => println!("Мы получили и сохранили значение: {}", val), // Выведет 120
        None => println!("Ничего не нашли"),
    }

    let dt_score = a.entry(String::from("Daria")).or_insert(0);
    *dt_score+=1;
println!("Final map: {:?}", a);
}

//?
fn get_daria_score(map:&HashMap<String, i32>)->Option<i32>{
    let score = map.get("Daria")?;
    Some(*score*10)
}

//
fn get_daria_score_match(map:&HashMap<String, i32>)->Option<i32>{
    let score = match map.get("Daria"){
        Some(num)=>num,
        None => return None,
    };
    Some(*score*10)
}