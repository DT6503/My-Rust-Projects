fn main() {
    let words = vec![String::from("rust"), String::from("is"), String::from("awesome")];

let new_vec: Vec<String> = words
.iter()
.map(|x| x.to_uppercase())
.collect();
    // Сделай итерацию так, чтобы каждое слово стало заглавным (UPPERCASE)
    // и сохрани результат в новый вектор.
    
    // ВАЖНО: После этого попробуй напечатать исходный вектор `words`.
    // Сделай два варианта:
    // 1. Где `words` остается доступен (используй .iter()).
    // 2. Где `words` поглощается (используй .into_iter()).

println!("{:?}", new_vec);
println!("{:?}", words);




}
