fn main() {
    println!("Hello, world!");

    let a = [1, 2, 3];
    let mut v:Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3];
    //v2.push(4); Gera erro. mut em variável emprestada como imutavel
    //  uma nova área de memória poderá ser usada, portanto v2 estará apontando
    //  para um local inválido.
    let second = &v[2];
    println!("O segundo elemento é {}", second);

    // Irá crashar pois não existe o index
    // uso de vetores traz o risco de travar em runtime
    //let four = &v[3];
    // Maneira segura:

    match v.get(4) {
        Some(four) => println!("O quarto elementro é {}", four),
        None => println!("Não existe quarto elemento")
    }

    let mut v3 = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        println!("v era {}", i);
        *i += 50;
    }
    for i in &v {
        println!("v agora é {}", i);
    }

    // Diferente tipos de dados
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Black")),
        SpreadsheetCell::Float(10.12)
    ];
    match &row[0] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Não é um valor inteiro")
    };
    println!("{:?}", row);
}
