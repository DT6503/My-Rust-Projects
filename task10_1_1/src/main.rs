
trait Speak{
    fn say(&self)->String;
}


struct Dog{
    voice: String,
}

impl Speak for Dog{
    fn say(&self)->String {
        println!("{}", String::from("Гав-гав"));
       //чисто, чтобы вернуть тип по задаче
        let str= String::from("Гав-гав");
        str
    }
}

struct Cat{
    voice: String,
}


impl Speak for Cat{
    fn say(&self)->String{
        //1ый способ
        //let str= String::from(&self.voice);
        //str

        //2ой способ
        self.voice.clone()
    }
}


fn main() {
    let cat = Cat{voice: String::from("Мяу-мяу")};
    println!("{}", cat.say());
let dog = Dog{voice: String::from("Гав-гав")};

dog.say();

}

