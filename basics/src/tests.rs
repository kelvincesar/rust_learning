const PI:f32 = 3.14159265359;
//static mut GLOBAL = 123;

fn funcao_01 () {
    let a = 123;
    // Novo escopo
    {
        let b = 456;
        println!("dentro a = {}", a);
        println!("dentro b = {}", b);
    }

    //println("fora b = {}", b);

    
}
