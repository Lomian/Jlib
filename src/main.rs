






fn add(a: &[i32], b: &[i32]) -> Vec<i32>  {

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x + y)
        .collect()


}




fn main() {

    let bor = add(&[1, 2, 3], &[1, 2, 3]);
    println!("{:?}", bor)


}