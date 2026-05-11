
use std::env;
use std::process; //выйти из проги без паники

use project12::Config;

fn main() {

    let args:Vec<String> = env::args().collect();

   // let query=&args[1];
   // let filename=&args[2];

   let config = Config::new(&args).unwrap_or_else(|err|{
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
   });

    //println!("{:?}", args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e)=project12::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

//run(config);

}
