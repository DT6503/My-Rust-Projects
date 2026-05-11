/**/
#[derive(Debug)]
struct Book{
  title: String,
author: String,
year: u32,
is_available: bool
}


impl Book {
    fn borrow (&mut self)
    {
self.is_available = false; 
    }


 fn return_book (&mut self)
    {
 self.is_available = true; 
    }

    fn display(&self){
        println!("We want to show you our Book: {:#?}", &self);
    }

}

impl Book {
    

 fn new(title:String, author:String, year: u32) -> Book{
       Book{
        title,
        author,
        year,
        is_available:true,
       } 
    }
}

fn main() {
   let mut book1 = Book::new(String::from("Master"), String::from("Bulgakov"), 1998,);

    let mut book2 = Book::new(String::from("Каблук"), String::from("Колас"), 1978,);

book1.display();
book2.display();

println!("////////////////");

book1.borrow();
book1.display();

println!("////////////////");

book1.return_book();
book1.display();

}
