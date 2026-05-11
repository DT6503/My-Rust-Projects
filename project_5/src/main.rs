


struct User{
username: String,
email:String,
sign_in_count:u64,
active:bool,
}



#[derive(Debug)]
struct Rectangle{
    width: u32,
    length: u32
}

impl Rectangle{
    fn area (&self)->u32{
        self.width*self.length
    }

    fn can_hold(&self, other:&Rectangle) -> bool{
        self.width>other.width && self.length>other.length
    }
}

impl Rectangle{
    fn squre (size: u32)->Rectangle{
       Rectangle{
    width: size,
    length: size
    }
    }
}

fn main() {

    let mut user1 = User{
        email: String::from("dt@gmail.com"),
        username: String::from("dt123"),
        active:true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("dt567");

    let user2 = build_user(
        String::from("ghg@nn.com"),
        String::from("hfg123"),
    );

    let user3 = User{
        email:String::from("ghg@nn.com"),
        username: String::from("ghg@nn.com"),
        ..user2
    };


    //Кортежные стр-ры
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

//Rectungl and square
let rect = Rectangle{
    width: 40,
    length: 60
};


println!("rect: {:#?}", rect);

println!(
    "The area of the rectangle is {} square pixels.",
   // area(&rect)
   rect.area()
);

let rect1 = Rectangle{
    width: 30,
    length: 70
};

let rect2 = Rectangle{
    width: 20,
    length: 30
};

let rect3 = Rectangle::squre(2);

println!("rect can hold rect1: {}", rect.can_hold(&rect1));
println!("rect can hold rect1: {}", rect.can_hold(&rect2));

}

fn build_user(email:String, username: String) -> User{
    User{
        email, //сокр синт-с иниц-ции полей
        username: username,
        active: true,
        sign_in_count:1,
    
    }
}

/*fn area(rectangle: &Rectangle)->u32{
    rectangle.width * rectangle.length 
}
*/


