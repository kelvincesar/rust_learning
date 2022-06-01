fn main() {
    println!("Ordena por inserção");
    let mut _A = vec![
        20, 25, 35, 40, 
        44, 55, 38, 99, 
        10, 65, 50
    ];
    println!("Vetor : {:?}", _A);
    ordena_por_insercao(&mut _A);

    println!("Vetor : {:?}", _A);

}


fn ordena_por_insercao(_A: &mut Vec<u16>){
    let n: usize = _A.len();
    println!("Size {}", n);
    for j in 1..n {
        println!("j = {}", j);


        let temp = _A[j];
        let mut i = j - 1;
        
        while i >= 1 && _A[i] > temp  {
            //println!("  [{} > {}] i = {}", A[i], temp, i);
            
            _A[i + 1] = _A[i];
            i = i-1;
            
            
        }
     

        _A[i + 1] = temp;
    }
}