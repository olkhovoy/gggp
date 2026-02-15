use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;

#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pyfunction]
#[pyo3(signature = (chromosome_a, chromosome_b, split=None))]
fn one_point_crossover(
    chromosome_a: Vec<i64>,
    chromosome_b: Vec<i64>,
    split: Option<usize>,
) -> PyResult<(Vec<i64>, Vec<i64>)> {
    let min_len = chromosome_a.len().min(chromosome_b.len());
    if min_len == 0 {
        return Ok((chromosome_a, chromosome_b));
    }

    let cross_at = split.unwrap_or(min_len / 2);
    if cross_at > min_len {
        return Err(PyIndexError::new_err(format!(
            "split={} is out of bounds for min_len={}",
            cross_at, min_len
        )));
    }

    let mut child_a = chromosome_a.clone();
    let mut child_b = chromosome_b.clone();
    child_a[cross_at..min_len].copy_from_slice(&chromosome_b[cross_at..min_len]);
    child_b[cross_at..min_len].copy_from_slice(&chromosome_a[cross_at..min_len]);
    Ok((child_a, child_b))
}

#[pyfunction]
#[pyo3(signature = (chromosome, index, value))]
fn mutate_gene(chromosome: Vec<i64>, index: usize, value: i64) -> PyResult<Vec<i64>> {
    if index >= chromosome.len() {
        return Err(PyIndexError::new_err(format!(
            "index={} is out of bounds for chromosome length={}",
            index,
            chromosome.len()
        )));
    }
    let mut out = chromosome;
    out[index] = value;
    Ok(out)
}

#[pymodule]
fn _core(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(version, module)?)?;
    module.add_function(wrap_pyfunction!(one_point_crossover, module)?)?;
    module.add_function(wrap_pyfunction!(mutate_gene, module)?)?;
    Ok(())
}

