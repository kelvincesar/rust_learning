// Rescursive enum:
// - rust can't know it size. use Box<> to fix it!
// - it will point to a fixed size on stack; 
// - box points to a variable size on heap
enum List {
    //Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    println!("Chapter 15 - Box pointer");

    /*
    when you have a type whose exact size
    can't be know at compile time
    and you want to use a value of that type
    in a conetext which requires knowing the exat size;

    when you have a larger amount of data and you
    want to transfer ownership of the data but you
    want to make sure that the data isn't copied

    when you own a value and you only care that the value
    implements a specific trait rather than it being a specific
    type (trait object ch17)
    */
    let b = Box::new(5);
    println!("B = {}", b);

    // Cons list
    //let list = Cons(1, Cons(2, Cons(3, Nil)));
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


}
