//Tuple Structs
#[derive(Debug)]
struct Color (i32, i32,i32);

#[derive(Debug)]
struct Point (i32, i32,i32);


fn main() {
    
let c1 = Color(0,1,1);
print_color(c1);


let c2 = Point(1,1,2);
//print_color(c1); как и обычные структуры с String или без Copy типов переместилось в ф-цию

}


fn print_color(c:Color){
    println!("{:#?}", c);
}