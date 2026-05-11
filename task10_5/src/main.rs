
fn compare_and_print<T:PartialEq>(a:T, b:T){
    if a==b{
        println!("Они одинаковые");
    }

    else {
        println!("Они разные");
    }
}

fn main() {
    compare_and_print("12", "1");
}
