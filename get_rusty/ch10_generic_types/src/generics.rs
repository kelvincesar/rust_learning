struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U>{
    fn x(&self) -> &T{
        &self.x
    }

    /*
    Altera o valor de y. Utiliza-se V e W pois deseja-se
    receber tipos de valores que possam ser diferentes dos tipos T e U.
    O retorno é T pois point.x não sofre alteração. W é retornado como novo tipo de número
    para o segundo registrador.
    */
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
} 
// Para ter uma função com tipo genérico, usa-se <T>
// <T> = <Type>
// Assim é possível alterar o Vec<i32> par Vec<T>
// PartialOrd e Copy indicam que o tipo genérico T
// deve ser uma estrutura que possa ser ordenada e copiada
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}


pub fn run () {
    println!("Running generics!\n");
    let number_list = vec![123,42, 11, 12332, 22, 131123132];

    let largest = get_largest(number_list);
    println!("O maior número da lista é {}", largest);

    let char_list = vec!["a", "d", "k", "l", "c"];
    let largest = get_largest(char_list);
    println!("O maior caracter da lista é {}", largest);


    let p1 = Point {x:5, y:1};
    let p2 = Point {x:5, y:1.232};

    /*
    enum Option<T> {
        Some(T), None,
    }
    enum Result<T, E>{
        Ok(T),
        Err(E),
    }
     */
    let p3 = Point {x: 5, y: 132.2};
    let p4 = Point {x: 123, y:"teste"};

    let p5 = p3.mixup(
        p4);

    println!("P.x = {}, P.y = {}", p5.x, p5.y);
}