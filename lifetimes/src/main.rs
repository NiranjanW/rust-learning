use std::collections::HashSet;

mod errors1;

fn find_element (vector: &Vec<String>, element:String ) -> bool{

        vector.into_iter().any(|el| *el == element)


}
//
// fn find_str( e: &String , v  : &mut Vec<&String>) ->String {
//     let words = v.into_iter()
//         .map(String::from)
//         .collect();
//     if words.contains(&e){
//         &e
//     }
//     println!("Not found");
//
//     let zoo_animals = vec!["Lion", "Zebra", "Giraffe", "Elephant", "Tiger"];
//     let target = "Giraffe";
//     match linear_search(&zoo_animals ,&target) {
//         Some(ind)
//     }
// }

// fn linear_search( v : Vec<String> , target : String) -> Option<usize>{
//
//     let found = v.iter().enumerate().find_map(|index, item| {
//             if item == target {
//                 Some(index)
//             } else {
//                 None
//             }
//         })
//     }


pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let h: HashSet<_> = nums.iter().copied().collect();
    nums.into_iter()
        .filter(|&x| !h.contains(&(x - 1)))
        .map(|x| (x..).take_while(|x| h.contains(x)).count())
        .max()
        .map_or(0, |x| x as i32)
}
    //  let inp: HashSet<&i32> = nums.iter().collect();
    //
    // inp.
    //     into_iter()
    //     .filter(|&x| !inp.conttains (x-1))
    //     .map(|x| ( (x..).take_while(|x| inp.containts(x).count())
    //     .max()
    //     .map_or(0, |x| x as i32)





fn find_needle( lst: Vec<&str> , ned: String)  {
    let haystack: Vec<_> = lst
        .into_iter()
        .map(String::from)
        .collect();
    if haystack.contains(&ned){
        println!("{}", ned);
    } else {
        println!("not found");
    }

}

fn is_odd(n:u32) -> bool{
    n %2 == 1
}
fn main() {
    let haystack1 : Vec<String> = vec!["a" , "list" , "of" , "large" , "string"].iter().map(|x|x.to_string()).collect();
    let haystack: Vec<_>  = vec!["a" , "list" , "of" , "large" , "strings"];
    find_needle(haystack , "strings".to_string() );
    find_element(&haystack1 , "strings".to_string());


        println!("Find the sum of all the squared odd numbers under 1000");
        let upper = 1000;

        // Functional approach
        let sum_of_squared_odd_numbers: u32 =
            (0..).map(|n| n * n)             // All natural numbers squared
                .take_while(|&n| n < upper) // Below upper limit
                .filter(|n| is_odd(*n))     // That are odd
                .fold(0, |sum, i| sum + i); // Sum them
        println!("functional style: {}", sum_of_squared_odd_numbers);

}

