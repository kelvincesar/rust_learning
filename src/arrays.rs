// Listas com tamanhos fixos e com datatypes iguais
use std::mem::size_of_val;

pub fn run() {
    let mut numbers: [i32; 5] = [0, 1, 2, 3, 4];
    


    // Unico valor
    println!("Valor da posição 0: {}", numbers[0]);

    numbers[3] = 33;

    println!("Array: {:?}", numbers);
    println!("Tamanho: {}", numbers.len());
    println!("Tamanho alocado na stack: {} bytes", size_of_val(&numbers));
    
    // Get slice 
    let slice: &[i32] = &numbers[2..];
    println!("Slice: {:?}", slice);


}