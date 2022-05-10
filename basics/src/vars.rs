// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

// As constantes não possuem um endereço de memória.
const PI:f32 = 3.15; // Const Funciona como #define
pub fn run() {
    // Variáveis não são mutáveis por padrão
    let name = "Kelvin";
    let mut age = 23;
    println!("Meu nome é {} e tenho {}", name, age);
    age = 24;
    println!("Meu nome é {} e tenho {}", name, age);

    println!("O valor de PI: {}", PI);
    let booleano: bool = true;

    println!("Variável booleana {}, tem tamanho {}", booleano, std::mem::size_of_val(&booleano));
    // Definindo uma constante
    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Múltiplas variáveis
    let ( meu_nome, minha_idade ) = ("Kelvin", "24");
    println!("{} = {}", meu_nome, minha_idade);

    let letra:char = 'K';
    println!("Tamanho do char = {}", std::mem::size_of_val(&letra));
}
