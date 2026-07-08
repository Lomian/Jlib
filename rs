fn add(a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32, i32)  {
    
    return (a.0 + b.0, a.1 + b.1, a.2 + b.2);
    
    
}




fn main() {

    let bor = add((3, 4 , 6), (4, 2, 3));
    println!("{:?}", bor)
    
    
}
