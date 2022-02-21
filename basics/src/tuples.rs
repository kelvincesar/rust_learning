// Tuples agrupam valores de diferentes data types
// Limitado em 12 elementos

pub fn run() {
    let person: (&str, &str, i8) = ("Kelvin", "Brasil", 24);

    println!("{} is from {} and is {}", person.0, person.1, person.2);    

}