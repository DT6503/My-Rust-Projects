
struct Counter {
    count:u32,
}


fn main() {

let mut c = Counter{count: 12};

//помечаем само замыкание как mut, так как оно реализует логику FnMut
    // (изменяет захваченное окружение при каждом вызове)
let mut increment = ||{
    c.count += 1;
    println!("{}", c.count);
};

   increment();
   increment();
   increment();
}
