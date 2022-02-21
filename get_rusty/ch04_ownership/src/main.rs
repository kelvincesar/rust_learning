/*
Each value in Rust has a variable that’s called its owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.

*/
fn main() {
    println!("Hello, world!");


    {// s não é válido, pois ainda n foi declarado
        let s = String::from("Olá!");   // s é valido e alocado na heap
        // do stuff

    } // o escopo termina, e s não é válido

    let x = 5;
    let y = x; // é feita a cópia

    // "Move", não é feito um shallow copy ou iniciada outra área de memória
    // s1 é deletado e s2 criado com os valores de s1
    let s1 = String::from("olá!");
    let s2 =  s1; 

    // para clonar
    // let s2 = s1.clone();
    

    // borrow
    let mut s = String::from("hello");
    // usa-se "&" pois logo em seguida a variável é usada
    // no caso, se passar sem "&", a variável é destruida 
    // pois é passado o "ownership"
    takes_ownership(&s);
    println!("{}", s);

    let z = 5;
    makes_copy(z);  // n precisa de ponteiro pois por ser inteiro já é executado um copy
    println!("{}", z);


    change(&mut s);
    println!("{}", s);

    // Não pode:
    //let r1 = &mut s;
    //let r2 = &mut s;
    //println!("{}, {}", r1, r2);
    let s2 = "hello world";
    let word = first_word(s2);
    s.clear();
    println!("the first word is: {}", word);
}


fn takes_ownership (some_string: &String) {
    println!("[takes_ownership] {}", some_string);
}

fn makes_copy (some_int: i32) {
    println!("[makes_copy] {}", some_int);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word (s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}