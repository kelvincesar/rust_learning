mod vectors;
mod structs;

fn main() {
    // casting de valor
    //let nota: f32 = 10f32;
    //let notas: [f32;4] = [6.4; 4]; => cria 4 posições com 6.4

    let notas:[f32;4] = [
        10f32,
        8f32,
        9.5,
        6.0
    ];
    let inteiro: usize = 0; // tem que usar usize para adequar a arquitura
    println!("{}", notas[inteiro]);

    for indice in 0..notas.len() {
        println!("Nota {} é {}",indice, notas[indice]);
    }
    matriz();
    let dia_atual = DiasSemana::SABADO;
    println!("é dia de semana? {}", is_weekend(dia_atual));

    cores();

    leitura_arquivo();

    vectors::run();

    structs::run();
}

// Remove warning do que não é usado
#[allow(dead_code)]
enum DiasSemana {
    DOMINGO,
    SEGUNDA,
    TERCA,
    QUARTA,
    QUINTA,
    SEXTA,
    SABADO
}

fn is_weekend(dia_semana: DiasSemana) -> bool {
    match dia_semana {
        DiasSemana::DOMINGO | DiasSemana::SABADO => true,
        _ => false
    }
}

fn matriz (){
    let matriz = [
        [0.0, 1.2, 0.1],
        [3.1, 312.2, 112.2]
    ];

    for linha in matriz {
        for item in linha {
            println!("item {} da linha", item);
        }
    }
}

// Remove warning do que não é usado
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black: u8}
}
fn cores() {

    let cor = Color::RgbColor(0, 120, 0);
    //let cor = Color::CymkColor { cyan: 100, magenta: 50, yellow: 70, black: 255 };

    println!("Cor = {}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "blue",
        Color::RgbColor(0, 0, 0) |
        Color::CymkColor { cyan: _, magenta: _, yellow: _, black: 255 } => "preto",
        Color::RgbColor(_, green, _) => {
            println!("RGB desconhecido: {}", green);
            "retornando"
        },
        Color::CymkColor { cyan: _, magenta: _, yellow: _, black: _ } => "CYMK desconhecido"
    });
}

fn leitura_arquivo (){
    let resposta = ler(String::from("/caminho"));

    match &resposta { 
        Some(conteudo) => println!("Conteúdo do arq: {}", conteudo),
        None => println!("Arquivo n encontrado")
    };

    println!("resposta arquivo debug {:?}", resposta);

    if let Some(valor) = resposta {
        println!("usando if let para mostrar valor do arquivo: '{}'", valor);
    }
    // while let... tbm existe
}
fn ler(caminho: String) -> Option<String> {
    Some(String::from("conteúdo do arquivo lido"))
   //None
}

/* 
    Generics:
        Utiliza o <T> pois "Some" pode ser u8, string, qualquer coisa...
        É um template, em que o compilador utilizará para copiar pelo código as de acordo com o que é utilizado.
        Exemplo:
enum Result<S, E> {
    Ok(S),
    Err(E)
}
*/