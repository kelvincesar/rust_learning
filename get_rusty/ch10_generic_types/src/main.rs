struct Point<T, U> {
    x: T,
    y: U
}

fn main() {
    let number_list = vec![123,42, 11, 12332, 22, 131123132];

    let largest = get_largest(number_list);
    println!("O maior número da lista é {}", largest);

    let char_list = vec!["a", "d", "k", "l", "c"];
    let largest = get_largest(char_list);
    println!("O maior caracter da lista é {}", largest);


    let p1 = Point {x:5, y:1};
    let p2 = Point {x:5, y:1.232};
}
// Para ter uma função com tipo genérico, usa-se <T>
// <T> = <Type>
// Assim é possível aterar o Vec<i32> par Vec<T>
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
