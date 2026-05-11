
enum Category {
    Electronica,
    Food,
    Clothing
}

struct Product{
    name:String,
    category: Category,
    price:f64,
}


trait Taxable {
    fn price_with_fax(&self) ->f64;
}

impl Taxable for Product{
    
}


fn main() {
    println!("Hello, world!");
}
