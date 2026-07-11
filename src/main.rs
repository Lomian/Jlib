use pyo3::prelude::*;


#[pyfunction]

fn define(a: Vec<i32>) {
    fn add(a: Vec<i32>, b: Vec<i32>) -> Vec<i32>  {

        a.iter()
            .zip(b.iter())
            .map(|(x, y)| x + y)
            .collect()


    }



}















fn main() {

    let bor = add(vec![1, 2, 3], vec![1, 2, 3]);
    println!("{:?}", bor)
    
    
}

#[pymodule]
fn jlib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(vec, m)?)?;
    Ok(())
}