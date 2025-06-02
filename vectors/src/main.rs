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

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut all_nums = (0..=upper_bound).collect::<Vec<u64>>();
    all_nums[1]=0;
    
    let stop = (upper_bound as f64).sqrt() as usize + 1usize;
    let upper_bound = upper_bound as usize;
    
    for i in 2..stop{
        if all_nums[i] != 0{
      // i is prime because it hasn't been marked as a multiple of any number
            for idx in (i * i..=upper_bound).step_by(i){
                all_nums[idx]=0
            }
        }
    }
    all_nums.iter().filter(|num| *num != &0u64).copied().collect()
}



