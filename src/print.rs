pub fn run() {
    println!("Olá do arquivo print.rs");

    println!("{} is from {}", "Kelvin", "Brasil");

    println!(
        "{0} is from {1} and likes to {2}", 
        "Kelvin", "Brasil", "play games"
    );

    println!(
        "{name} likes to ride his {sport}",
        name = "Kelvin",
        sport = "bike"
    );

    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    println!("{:?}", (10+12, "Kelvin", true) );
}