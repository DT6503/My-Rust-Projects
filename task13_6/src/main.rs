


fn main() {

let mut report = String::from("Отчет: ");

let mut add_record = |x| {
    report.push_str(x);
};

add_record("первый и последний");
    //println!("{}", report);

    add_record("JUJ");
    println!("{}", report);
}
