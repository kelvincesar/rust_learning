use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;


fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // Tentativa de leitura do arquivo. Caso ocorra um erro, a função é retornada
    // indicando o erro
    let mut f = File::open("hello.txt")?;
    
    // O mesmo ocorre para essa função
    f.read_to_string(&mut s)?;

    // Caso tenha chego até aqui, entende-se que tudo ocorreu bem
    // retorna o valor lido.

    // Poderia ser simplificado em uma linha:
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
fn main() {
    println!("Hello, world!");
    // Tenta abrir o arquivo
    let f = File::open("hello.txt");

    // Valida o retorno
    let f = match f {
        Ok(file) => file,
        // Caso n tenha encontrado o arquivo, tenta cria-lo
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problema ao criar arquvio {:?}", e),

            },
            // Valida também erro ao tentar cria-lo
            other_error =>{
                panic!("Fechando!!! {:?}", other_error)
            }
        }
    };

    // Outra maneira de validar...
    let f = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error|{
                panic!("Problema ao criar o arquivo {:?}", error);
            })
        } else {
            panic!("Problema ao abrir o arquivo: {:?}", error);
        }
    });

    let f = File::open("hello3.txt").expect("Problema ao abrir o arquivo hello3.txt");
}


