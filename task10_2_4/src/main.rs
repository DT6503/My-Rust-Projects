
//fn func1(s: &str) -> &str;//сам тк lifetime входного=выходного
//fn func2(s1: &str, s2: &str) -> &str; //надо самим расставлять лайфтаймы
//fn func3(&self, announcement: &str) -> &str; //сам расставит


fn main() {
    
let s1 = "AAA";
{
let s2 = "B";


}

println!("{}", longest(s1, s2));

println!("{}, {}", s1, s2);

}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}