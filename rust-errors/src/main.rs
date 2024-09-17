use std::fs::File;
use std::io::{self , Read};
use std::env::args;

fn main() {
   
    let args : Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let file_name = &args[1];
    match read_file(file_name){
        Ok(content) =>  println!("{:?}" , content),
        Err(err) => eprintln!("Error reading file: {}", err),
   
    }
    
   
}

fn read_file(name : &str) -> Result<String , io::Error> {
     let mut file_handler = File::open(name)?;
     let mut contents = String::new();
     file_handler.read_to_string(&mut contents)?;
     Ok(contents)


}