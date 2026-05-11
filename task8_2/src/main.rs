
fn get_element(v:&Vec<i32>, index:usize){
    match v.get(index)    
    {
Some(value)=>println!("Вектор: {}", value),
None=>println!("Ошибка: индекс {} вне диапазона", index),
    }
}


fn main() {
    let mut a:Vec<i32> = Vec::new();
    a.push(2);
    a.push(3);
    a.push(5);

    get_element(&a, 1);
 get_element(&a, 3);

let mut b = vec![12, 23, 45];
println!("B: {:?}", &b);

let c = vec![1;10];
println!("C: {:?}", &c);
}
