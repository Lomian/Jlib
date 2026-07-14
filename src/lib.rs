use pyo3::prelude::*;


#[pyfunction]

fn vec(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {


    //  println!("{:?}", a);
    a

}


fn shape(a: Vec<Vec<i32>>) {
    let mut count = 0;
    let amount = a[0].len();
    for listsi in a {

        count += 1;

        //println!("{:?}", listsi);


    }
    println!("({}, {})", count, amount);
}









fn main() {
    let emmen = vec(vec![vec![12, 12, 22], vec![1, 2, 3]]);

    shape(emmen)


}

#[pymodule]
fn jlib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(vec, m)?)?;
    Ok(())
}