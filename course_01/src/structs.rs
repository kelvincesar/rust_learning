struct Titular {
    nome: String,
    idade: u8
}
struct Conta {
    titular: Titular,
    saldo: f64
}

impl Conta {
    fn sacar (&mut self, valor:f64){
        self.saldo -= valor;
    }
}

pub fn run () {
    println!("\n::STRUCTS::");

    let mut conta = Conta {
        titular: Titular { 
            nome: String::from("Kelvin Andrade"),
            idade: 24
        },
        saldo: 100.2
    };
    conta.sacar(50.0);
    println!("Dados da conta:");
    println!("Nome = {}, idade = {}", conta.titular.nome, conta.titular.idade);
    println!("Saldo = {}", conta.saldo);
}