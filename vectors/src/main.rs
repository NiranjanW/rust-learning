// https://medium.com/rustaceans/rust-vectors-a-fun-guide-with-examples-ba9402139e5a

enum MiddleEarth {
    Human(String),
  Elf(String),
  Dwarf(String),
  Hobbit(String),
}

fn main() {
    let  v = vec!(1,2,3);
    let squares: Vec<_> = v.iter()
        .map(|x| x * x) 
        .filter(|&x| x > 2)
        .collect();
    for c in squares {
        println!("{}", c);
    }
    vec_slice();

    let council_members = vec![
        MiddleEarth::Human("Frodo".to_string()),
        MiddleEarth::Elf("Legolas".to_string()),
    ];

    for member in council_members {
        match member {
            MiddleEarth::Human(name) => println!("Hello, {}!", name),
            MiddleEarth::Elf(name) => println!("Bye, {}!", name),
            MiddleEarth::Dwarf(name) => println!("{}!", name),
            MiddleEarth::Hobbit(name) => println!("{}!", name),
        }
    }
   

}

fn vec_slice() {
    let v = vec![1,2,3,4,5];
    let slice = &v[1..4];
    println!("{:?}", slice);
}   