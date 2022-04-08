use pyo3::prelude::*;
use datapackage_convert as datapackage_convert_rs;

#[pyfunction]
fn merge_datapackage(output_path: String, datapackages: Vec<String>) -> eyre::Result<()> {
    datapackage_convert_rs::merge_datapackage(output_path.into(), datapackages)?; 
    Ok(())
}

#[pyfunction]
fn datapackage_to_sqlite(db_path: String, datapackage: String) -> eyre::Result<()> {
    datapackage_convert_rs::datapackage_to_sqlite(db_path.into(), datapackage)?; 
    Ok(())
}

#[pyfunction]
fn datapackage_to_parquet(output_path: String, datapackage: String) -> eyre::Result<()> {
    datapackage_convert_rs::datapackage_to_parquet(output_path.into(), datapackage)?; 
    Ok(())
}

#[pymodule]
fn datapackage_convert(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(merge_datapackage, m)?)?;
    m.add_function(wrap_pyfunction!(datapackage_to_sqlite, m)?)?;
    m.add_function(wrap_pyfunction!(datapackage_to_parquet, m)?)?;
    Ok(())
}



