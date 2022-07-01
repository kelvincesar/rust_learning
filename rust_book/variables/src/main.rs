fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("valor de x é {}", x);
    x = 6;
    println!("valor de x é {}", x);

    // Shadowing
    let x = "uma string";
    println!("valor de x é {}", x);
    // Não pode ser mutável
    const UMA_CONSTANTE: u32 = 10000;

    // Tuples:
    let tup = ("Rust!", 12_000);
    let lang = tup.0;

    println!("O valor da posição 0 {} e da constante {}", lang, UMA_CONSTANTE);

    // loop

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter >= 10 {
            break counter;
        }
    };
    println!("Valor do contador ao final do loop {}", result);

    // outro loop
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("valor é: {}", element);
    }

    for number in 1..5 {
        println!("{}!", number);
    }




}
