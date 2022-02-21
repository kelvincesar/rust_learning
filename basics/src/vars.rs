// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    // Variáveis não são mutáveis por padrão
    let name = "Kelvin";
    let mut age = 23;
    println!("Meu nome é {} e tenho {}", name, age);
    age = 24;
    println!("Meu nome é {} e tenho {}", name, age);

    // Definindo uma constante
    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Múltiplas variáveis
    let ( meu_nome, minha_idade ) = ("Kelvin", "24");
    println!("{} = {}", meu_nome, minha_idade);
}