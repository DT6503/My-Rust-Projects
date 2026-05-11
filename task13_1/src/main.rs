fn main() {
    let locker = |x| x;

    let s = locker(String::from("Rust"));
    let n = locker(5); // Почему здесь будет ошибка и как ее решить?
/*
потому что вначале уже определо было что тип - строка
*/

let example = |x| println!("{}", x);
example(10);
example(20);

}
