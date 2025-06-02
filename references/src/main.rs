// https://blog.devgenius.io/how-to-master-rust-references-in-just-10-minutes-214c89d81f9f
struct Container(Vec<u64>);
// struct Container {
//     vec: Vec<u64>,
// }
use std::collections::BTreeMap;
impl Container {
    fn get(&self, index: usize) -> &u64 {
        &self.0[index]
    }
}


fn transfor ( map: &BTreeMap<i32 , Vec<char>) -> BTreeMap<char , i32>{
    let mut result:BTreeMap<char , i32> = BTreeMap::new();
    for (key, values) in map {
        for value in values {
            result.insert(value.to_ascii_lowercase(), *key);
        }
    }
    result
}
fn main() {
    let mut a = 5;
    let mut b = &a;
    /  l/et c = &mut a;
    let c = &mut b;
    **c += 1;
    println!("value {}  ", *c);
    // let m = Container(vec![1, 2, 3]);
    //  let mut the_ref = m.get(0);
    //  {
    //      let d = Container(vec![1, 2, 3]);
    //      the_ref = d.get(1);
    //  }
    // println!("{}", the_ref);
    let str = String::from("Hello World");
    println!("{}", first_words(&str))
}

fn first_words(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
