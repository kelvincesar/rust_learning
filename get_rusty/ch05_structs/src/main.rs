struct User {
    email: String,
    username: String,
    active: bool,
    count: u32
}
fn new_user (email: String, username: String) -> User {
    User { 
        email,
        username,
        active: true,
        count: 1
    }
}

#[derive(Debug)] // permite uso do print na estrutura
struct Retangulo {
    largura: f32,
    comprimento: f32
}
// implementações para a estrutura retangulo
impl Retangulo {
    fn area (&self) -> f32 {
        self.largura * self.comprimento
    }

    fn compare (&self, other: &Retangulo) -> bool {
        self.area() > (other.largura * other.comprimento)
    }
    // Poderia ser implementado em outra "impl" separada, não muda nada...
    fn square (size: f32) -> Retangulo {
        Retangulo {
            largura: size,
            comprimento: size
        }
    }
}
fn area (ret: &Retangulo) -> f32 {
    ret.largura * ret.comprimento
}

fn main() {
    println!("Using structs!");

    let user_1 = User {
        email: String::from("kelvin@gmail.com"),
        username: String::from("kca"),
        active: true,
        count: 1
    };

    let user_2 = new_user(
        "kelvinc@hotmail.com".to_string(), 
        "kelvinc".to_string()
    );

    let user_3 = User {
        email: String::from("cesar@gmail.com"),
        username: String::from("cesar"),
        ..user_2
    };

    let ret = Retangulo{
        largura: 12.0,
        comprimento: 12.0
    };

    let ret_2 = Retangulo{
        largura: 11.0,
        comprimento: 11.0
    };
    let ret_3 = Retangulo{
        largura: 15.0,
        comprimento: 15.0
    };

    let ret_4 = Retangulo::square(12.2);
    println!(" A dimensão do retângulo é: {:#?}", ret);

    println!(" A área do retângulo 1 é: {} pixels ", area(&ret));
    println!(" A área do retângulo 1 é: {} pixels ", ret.area());

    println!(" Retangulo 1 > 2? {}", ret.compare(&ret_2));
    println!(" Retangulo 1 > 3? {}", ret.compare(&ret_3));
    println!(" Retangulo 1 > 4? {}", ret.compare(&ret_4));
}



