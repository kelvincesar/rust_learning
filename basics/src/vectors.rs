// Vectors são arrays que podem ter seu tamanho alterado
use std::mem::size_of_val;

pub fn run() {
    let mut numbers: Vec<i32> = vec![0, 1, 2, 3, 4];
    


    // Unico valor
    println!("Valor da posição 0: {}", numbers[0]);

    numbers[3] = 33;
    println!("Vector: {:?}", numbers);
    // Adicionar em um vetor
    numbers.push(5);
    numbers.push(6);
    println!("Vector: {:?}", numbers);
    numbers.pop();
    println!("Vector: {:?}", numbers);

    println!("Tamanho: {}", numbers.len());
    println!("Tamanho alocado na stack: {} bytes", size_of_val(&numbers));
    
    // Get slice 
    let slice: &[i32] = &numbers[2..];
    println!("Slice: {:?}", slice);

    // Loop em vetores
    for x in numbers.iter() {
        println!("Número: {}", x);
    }
    // Loop em vetores alterando
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Vector: {:?}", numbers);
}