
//"Slices in Action"

fn main() {
    let mut s = String::from("Hello World");
    let word = find_first_word(&s);
    //s.clear();
    println!("{}", word);
}


fn find_first_word(s: &str)->&str{
let bytes = s.as_bytes();

for (i, &item) in bytes.iter().enumerate(){
    if item == b' '{
        return &s[0..i];
}
}
    s
}