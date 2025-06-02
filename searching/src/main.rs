use std::cmp::Ordering;
fn greet ( first : &str, last :&str) {
    println!("Hello {first} {last}")
}

fn get_name(name1 :String , name2 :String) -> (String , String) {
    (name1 ,name2 )
}

fn binary_search(arr : &[i32] , key : i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let nodes =arr;
    while low <= high {
        let mid = (low + high) / 2;
        match nodes[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Greater => high = mid - 1,
            Ordering::Less => low = mid + 1,
        }
    }
  
    
    // while low <= high {
    //     let mid = (low + high) / 2;
    //     if arr[mid] == key {
    //         return Some(mid);
    //     } else if arr[mid] < key {
    //         low = mid + 1;
    //     } else {
    //         high = mid - 1;
    //     }

    // }
    None
}
fn main() {
    let name1 = String::from("Niranjan");
    let name2 = String::from("Wijeyanathan");
    let(mut first , mut last) = get_name(name1, name2);
    greet("Narain" , "Wijeyanathan" );
    // greet(&name1, &name2);


    // Mutable references
    let mut name = String::from ("Richi");
    let n = &mut name;
    *n = String::from("Claire");
    println!("{name}");


    let arrays = vec![4, 8, 12, 16, 23, 28, 32];
    
    assert_eq!(binary_search(&arrays,23), Some(4));
    assert_eq!(binary_search(&arrays,12), Some(2));
}