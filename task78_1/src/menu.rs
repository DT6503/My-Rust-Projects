
pub struct Food{
pub name:String,
pub price:f64,
}


impl Food{
    pub fn new(name:String, price:f64)->Self{
        Self{name, price}
    }
}
