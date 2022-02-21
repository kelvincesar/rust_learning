pub fn run() {
    let mut hello = String::from("Hello ");

    println!("Tamanho: {}", hello.len());

    hello.push('W');
    hello.push_str("orld!");

    println!("Palavra: {}", hello);

    println!("Capacidade: {} bytes", hello.capacity());

    println!("Está vázio?: {}", hello.is_empty());

    println!("Contém 'World' {}", hello.contains("World"));

    println!("Replace: {}", hello.replace("World", "There"));

    println!("Looping>>");
    // Loop separando as palavras por espaços
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    println!("{}", s);
    // Cria string com capacidade

}