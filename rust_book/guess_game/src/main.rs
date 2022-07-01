use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("{}", "Bem-vindo ao jogo de adivinhação!".blink());
    let limits:(u32, u32) = (1, 10);
    let secret_number:u32  = rand::thread_rng().gen_range(limits.0, limits.1);
    loop {
        println!("\n* Tente advinhar um número entre ({low} e {high}): ", low=limits.0, high=limits.1);


        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Erro ao ler a linha");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("O valor digitado não é válido");
                continue;
            }
        };
    
    
        println!("Você digitou: {}", guess);
       
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "O número digitado é menor.".red()),
            Ordering::Greater => println!("{}", "O número digitado é maior.".red()),
            Ordering::Equal => {
                println!("{}", "Você venceu!!".green());
                break;
            },
        }
    }
    
}
