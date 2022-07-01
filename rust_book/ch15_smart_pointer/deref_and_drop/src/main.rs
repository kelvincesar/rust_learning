use std::ops::Deref;

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// allow compiler to deref MyBox...?

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Droping CustomSmartPointer with data <{}>", self.data);
    }

}

fn main() {
    let x = 5;
    let y = &x; // reference to x

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // cannot cmpare a integer to a reference of an integer
    //assert_eq!(5, y);

    let z = Box::new(x);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    //hello without deref (if rust didn't have it)
    //hello(&(*m)[..]);
    // DerefMut...

    println!("DROP");
    
    let C = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // From rust preludej 
    drop(d);
    println!("Custom Smart Pointer created.");
}


fn hello(name: &str) {
    println!("Hello, {}!", name);
}