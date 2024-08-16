use std::fs::File;
mod shape;

// use stdio::prelude::*;
// use std::io::BufReader;
use std::env;
use serde::{Deserialize, Serialize};
use serde_json::Value;

fn main() {
    let range : Vec<u32> = (0..10).collect();
    let num = env::args().skip(1);
    let file_path = env::var("FILE_PATH").expect("FILE_PATH must be set");
    let file_content =
        std::fs::read_to_string(file_path).expect("FILE_PATH environment variable not set");
    // println!("{}", file_content);
    let parsed_date : serde_json::Value = serde_json::from_str(&file_content).unwrap();
    println!("{}", parsed_date);

    if let Some(value) = parsed_date.get("fiscalPeriod") {
        println!("{}", value);
    }else {
        println!("fiscalPeriod not found");
    }
let mut name = String::from("Niranjan");
let n = &name;
let n1 = &mut name;
*n1 = String::from("Narain");
println!("(n)(n1)");

let arry = [1,3,5,2,7];

arry.iter().filter(|&&x| x >2).for_each(|&x| println!("{}",x))

}


fn binary_search< T : Ord + PartialEq > ( arry : &[T] , element : &T ) -> Option<usize>{
    let mut low = 0;
    let mut high = arry.len() -1;

    while low <= high {
        let mid = (low +high ) /2;
        if *arry.get(mid).unwrap() == *element{
            return Some(mid);
        }else if *arry.get(mid).unwrap() < *element{
            low = mid +1;
        }else {
            high = mid -1;
        }

        

    }
None

}
fn get_nested_value(obj: &Value, key_path: &str) -> Option<&Value> {
    let keys: Vec<&str> = key_path.split('.').collect();

    let mut current_value = obj;
    for key in keys {
        current_value = current_value.get(key)?;
    }

    Some(current_value)
}

fn gcd ( mut a : i32, mut b : i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
