use pyo3::prelude::*;


#[pyfunction]

fn vec(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {


  //  println!("{:?}", a);
    a

}


fn add(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) {



}









fn main() {
    vec(vec![vec![12, 12, 22]]);




}

#[pymodule]
fn jlib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(vec, m)?)?;
    Ok(())
}