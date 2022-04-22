use std::fs::File;
use std::io::ErrorKind;


fn main() {
    println!("Hello, world!");

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problema ao criar arquvio")

            },
            other_error =>{
                panic!("Fechando!!! {?}", other_error);
            }
        }
    }
}
