use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Leitura dos argumentos
    let config = Config::new(env::args()).unwrap_or_else(|err| {        
        eprintln!("Erro ao ler os argumentos: {}", err);
        process::exit(127);
    });

    println!("Procurando pela palavra: '{}'", config.query);
    println!("No arquivo: '{}'\n", config.file);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(127);
    }
    
    

}

