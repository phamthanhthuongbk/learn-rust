use pyo3::prelude::*;

fn fibonacci_py(n: i64) -> PyResult<i64> {
    Python::with_gil(|py| {
        let fun = PyModule::from_code_bound(
            py,
            r#"
import time
def fibonacci(n):
    print('fibonacci')
    print('fibonacci')
    print('fibonacci')
    time.sleep(3)
    a, b = 1, 0
    for _ in range(n):
        a, b = b, a + b
    return b
            "#,
            "",
            "",
        )?
        .getattr("fibonacci")?;

        fun.call1((n,))?.extract()
    })
}


pub fn main() {
    let result = fibonacci_py(10).unwrap();
    println!("result: {}", result);
}