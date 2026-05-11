
use std::error::Error;
use std::fs;
use std::env;


pub fn run(config:Config)->Result<(), Box<dyn Error>>
{
let contents = fs::read_to_string(config.filename)?;
    

let results = if config.case_sensitive{
    search(&config.query, &contents)
}    else{
search_case_insensitive(&config.query, &contents)
    };


    for line in results{
 println!("{}", line);

    }
   
    Ok(())

}

pub struct Config
{
   pub query:String,
   pub filename:String,
   pub case_sensitive:bool,
}

impl Config {
  pub fn new (args:&[String])->Result<Config,&str>{

if args.len()<3{
 //  panic!("not enough arguments!");
 return Err("not enough arguments!");
   
}

    let query = args[1].clone(); //чтобы не забрать строку
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();//переменная окр-ия

   Ok(Config { query, filename, case_sensitive})
}  
}


pub fn search<'a>(query: &str, contents:&'a str)->Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}


pub fn search_case_insensitive<'a>(query: &str, contents:&'a str)->Vec<&'a str>{

let query = query.to_lowercase();//shadowing
let mut results = Vec::new();

for line in contents.lines(){
    if line.to_lowercase().contains(&query){
        results.push(line);
    }
}
results
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
let query = "blaaaaaa,";
let contents = "\
Blaфффф,
blaaaaaa,
Bla
bla";

assert_eq!(vec!["blaaaaaa,"], search(query, contents));

    }



    #[test]
    fn case_insensitive(){
        let query = "blAaaaAa,";
let contents = "\
Blaфффф,
Blaaaaaa,
Bla
bla
blaaaaaa,";

assert_eq!(vec!["Blaaaaaa,", "blaaaaaa,"], search_case_insensitive(query, contents));

    }
}