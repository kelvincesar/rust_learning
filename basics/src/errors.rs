pub fn run() {
    let v = vec![12, 23, 123];
    //let x = v[4];

    match resultado(false) {
        Ok(s) => println!("String de successo: ´{}´", s),
        Err(codigo_erro) => println!("Codig do erro {}", codigo_erro)
    };
}

fn resultado(erro:bool) -> Result<String, u8> {
    match erro {
        true => Ok(String::from("deu certo")),
        false => Err(127)
    }
}