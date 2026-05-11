fn main() {
// 1 способ
  /*let s1 = String::from("HI");  
   let s2 = process_string(s1);


}

fn process_string(input:String) -> String {
    println!("{input}");
    return input*/



    // 2 способ
  let s1 = String::from("HI");  
   let s2 = process_string(&s1);


}

fn process_string(input:&String) -> &String {
    println!("{input}");
    return input
}