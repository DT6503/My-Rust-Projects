trait Speak {
    fn say(&self) -> String{
        String::from("...тишина...")
    }
}

struct Dog {
    voice: String,
}

impl Speak for Dog {
    fn say(&self) -> String {
        self.voice.clone()  
    }
}

struct Cat {
    voice: String,
}

impl Speak for Cat {
    fn say(&self) -> String {
        self.voice.clone() 
    }
}

struct Fish{
    voice: String,
}

impl Speak for Fish{

}

//trait as a parameter
fn make_it_speak(item: &impl Speak){
   println!("{}", item.say());
}


fn main() {
    let cat = Cat { voice: String::from("Мяу-мяу") };
    let dog = Dog { voice: String::from("Гав-гав") };
    
   // println!("{}", cat.say());
    //println!("{}", dog.say());

let fish = Fish{voice: String::from("Буль-буль")};
//println!("{}", fish.say());

make_it_speak(&cat);
make_it_speak(&dog);
make_it_speak(&fish);

}