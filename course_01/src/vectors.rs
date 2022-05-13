pub fn run (){
    println!("\n::VECTORS::");
    // Cria vetor com certa capacidade na heap
    let mut notas:Vec<f32> = Vec::new();
    /*
    Alocar memória na heap é um processo custoso, 
    então evitar esse processo é necessário. 
    PS.: Além de alocar a memória, esse processo 
    também demanda a cópia dos dados de um endereço 
    para outro (o que não é tão custoso assim).
    */
    let notas_3: Vec<f32> = Vec::with_capacity(4);

    println!("Capacidade notas = {}", notas.capacity());
    println!("Capacidade notas_3 = {}", notas_3.capacity());
    // Adiciona valores
    notas.push(10.4);
    notas.push(13.21);
    // Capacidade aumenta para '4'
    // o push aloca 3 posições na heap
    // para evitar buscar alocação em próximo push;
    println!("Capacidade notas = {}", notas.capacity());
    // Outra maneira de criar vetores com valors pré-definidos
    let mut notas_2 = vec![10.4, 13.21, 2.2];
    notas_2[0] += 1.2;
    

    
    println!("Nota 1 = {}", notas[0]);

    println!("Nota 6 = {}", match notas.get(7) {
        Some(n) => *n,  // Desreferencia o valor de n
        None => 0.0
    });

    // remover ultimo valor
    // necessário usar if/while let quando for usar o valor, por ser um option...
    while let Some(nota) = notas.pop(){
        println!("Último valor: {}", nota);
    }


    // necessário fazer borrowing '&' p/ nao perder o valor
    for nota in &notas {
        println!("Nota_2 = {}", nota);
    }

    println!("Notas: {:?}", notas);  
}