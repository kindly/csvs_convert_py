use pyo3::prelude::*;
use pyo3::types::PyDict;
use datapackage_convert as datapackage_convert_rs;
use datapackage_convert::Options;

#[pyfunction(kwds="**")]
fn merge_datapackage(output_path: String, datapackages: Vec<String>, kwds: Option<&PyDict>) -> eyre::Result<()> {
    let mut options = Options::builder().build();

    if let Some(kwds) = kwds {
        if let Some(delete) = kwds.get_item("delete_input_csv") {
            if let Ok(delete_input_csv) = delete.extract::<bool>() {
                options.delete_input_csv = delete_input_csv
            }
        }
    }
    
    datapackage_convert_rs::merge_datapackage_with_options(output_path.into(), datapackages, options)?;
    Ok(())
}

#[pyfunction(kwds="**")]
fn datapackage_to_sqlite(db_path: String, datapackage: String, kwds: Option<&PyDict>) -> eyre::Result<()> {
    let mut options = Options::builder().build();

    if let Some(kwds) = kwds {
        if let Some(delete) = kwds.get_item("delete_input_csv") {
            if let Ok(delete_input_csv) = delete.extract::<bool>() {
                options.delete_input_csv = delete_input_csv
            }
        }
    }
    datapackage_convert_rs::datapackage_to_sqlite_with_options(db_path.into(), datapackage, options)?; 
    Ok(())
}

#[pyfunction(kwds="**")]
fn datapackage_to_parquet(output_path: String, datapackage: String, kwds: Option<&PyDict>) -> eyre::Result<()> {
    let mut options = Options::builder().build();

    if let Some(kwds) = kwds {
        if let Some(delete) = kwds.get_item("delete_input_csv") {
            if let Ok(delete_input_csv) = delete.extract::<bool>() {
                options.delete_input_csv = delete_input_csv
            }
        }
    }
    datapackage_convert_rs::datapackage_to_parquet_with_options(output_path.into(), datapackage, options)?; 
    Ok(())
}

#[pyfunction(kwds="**")]
fn datapackage_to_xlsx(xlsx_path: String, datapackage: String, kwds: Option<&PyDict>) -> eyre::Result<()> {
    let mut options = Options::builder().build();

    if let Some(kwds) = kwds {
        if let Some(delete) = kwds.get_item("delete_input_csv") {
            if let Ok(delete_input_csv) = delete.extract() {
                options.delete_input_csv = delete_input_csv
            }
        }

        if let Some(use_titles) = kwds.get_item("use_titles") {
            if let Ok(use_titles) = use_titles.extract() {
                options.use_titles = use_titles
            }
        }

        if let Some(seperator) = kwds.get_item("seperator") {
            if let Ok(seperator) = seperator.extract() {
                options.seperator = seperator
            }
        }
    }

    datapackage_convert_rs::datapackage_to_xlsx_with_options(xlsx_path.into(), datapackage, options)?; 
    Ok(())
}

#[pymodule]
fn datapackage_convert(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(merge_datapackage, m)?)?;
    m.add_function(wrap_pyfunction!(datapackage_to_sqlite, m)?)?;
    m.add_function(wrap_pyfunction!(datapackage_to_parquet, m)?)?;
    m.add_function(wrap_pyfunction!(datapackage_to_xlsx, m)?)?;
    Ok(())
}