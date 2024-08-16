fn greet ( first : &str, last :&str) {
    println!("Hello {first} {last}")
}

fn get_name(name1 :String , name2 :String) -> (String , String) {
    (name1 ,name2 )
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
    println!("{name}")
}