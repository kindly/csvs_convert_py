use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use csvs_convert as csvs_convert_rs;
use csvs_convert::Options;

fn kwds_to_options(kwds: Option<&PyDict>) -> Options {
    let mut options = Options::builder().build();
    if let Some(kwds) = kwds {
        if let Some(delete) = kwds.get_item("delete_input_csv") {
            if let Ok(delete_input_csv) = delete.extract::<bool>() {
                options.delete_input_csv = delete_input_csv
            }
        }
        if let Some(drop) = kwds.get_item("drop") {
            if let Ok(drop) = drop.extract::<bool>() {
                options.drop = drop
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
        if let Some(schema) = kwds.get_item("schema") {
            if let Ok(schema) = schema.extract() {
                options.schema = schema
            }
        }
        if let Some(evolve) = kwds.get_item("evolve") {
            if let Ok(evolve) = evolve.extract() {
                options.evolve = evolve
            }
        }
        if let Some(double_quote) = kwds.get_item("double_quote") {
            if let Ok(double_quote) = double_quote.extract() {
                options.double_quote = double_quote
            }
        }
        if let Some(quote) = kwds.get_item("quote") {
            if let Ok(quote) = quote.extract() {
                options.quote = quote
            }
        }
        if let Some(delimiter) = kwds.get_item("delimiter") {
            if let Ok(delimiter) = delimiter.extract() {
                options.delimiter = delimiter
            }
        }
        if let Some(stats) = kwds.get_item("stats") {
            if let Ok(stats) = stats.extract() {
                options.stats = stats 
            }
        }
        if let Some(stats_csv) = kwds.get_item("stats_csv") {
            if let Ok(stats_csv) = stats_csv.extract() {
                options.stats_csv = stats_csv
            }
        }
        if let Some(threads) = kwds.get_item("threads") {
            if let Ok(threads) = threads.extract() {
                options.threads = threads
            }
        }
    }
    options
}

#[pyfunction(kwds="**")]
fn merge_datapackage(output_path: String, datapackages: Vec<String>, kwds: Option<&PyDict>) -> eyre::Result<()> {
    let options = kwds_to_options(kwds);
    csvs_convert_rs::merge_datapackage_with_options(output_path.into(), datapackages, options)?;
    Ok(())
}

#[pyfunction(kwds="**")]
fn datapackage_to_sqlite(db_path: String, datapackage: String, kwds: Option<&PyDict>) -> eyre::Result<()> {
    let options = kwds_to_options(kwds);
    csvs_convert_rs::datapackage_to_sqlite_with_options(db_path.into(), datapackage, options)?; 
    Ok(())
}

#[pyfunction(kwds="**")]
fn csvs_to_sqlite(db_path: String, csvs: Vec<PathBuf>, kwds: Option<&PyDict>) -> eyre::Result<String> {
    let options = kwds_to_options(kwds);
    let output = csvs_convert_rs::csvs_to_sqlite_with_options(db_path.into(), csvs, options)?; 
    Ok(output.to_string())
}

#[pyfunction(kwds="**")]
fn datapackage_to_postgres(postgres_url: String, datapackage: String, kwds: Option<&PyDict>) -> eyre::Result<()> {
    let options = kwds_to_options(kwds);
    csvs_convert_rs::datapackage_to_postgres_with_options(postgres_url.into(), datapackage, options)?; 
    Ok(())
}

#[pyfunction(kwds="**")]
fn csvs_to_postgres(postgres_url: String, csvs: Vec<PathBuf>, kwds: Option<&PyDict>) -> eyre::Result<String> {
    let options = kwds_to_options(kwds);
    let output = csvs_convert_rs::csvs_to_postgres_with_options(postgres_url.into(), csvs, options)?; 
    Ok(output.to_string())
}

#[pyfunction(kwds="**")]
fn datapackage_to_parquet(output_path: String, datapackage: String, kwds: Option<&PyDict>) -> eyre::Result<()> {
    let options = kwds_to_options(kwds);
    csvs_convert_rs::datapackage_to_parquet_with_options(output_path.into(), datapackage, options)?; 
    Ok(())
}

#[pyfunction(kwds="**")]
fn csvs_to_parquet(output_path: String, csvs: Vec<PathBuf>, kwds: Option<&PyDict>) -> eyre::Result<String> {
    let options = kwds_to_options(kwds);
    let output = csvs_convert_rs::csvs_to_parquet_with_options(output_path.into(), csvs, options)?; 
    Ok(output.to_string())
}

#[pyfunction(kwds="**")]
fn csvs_to_xlsx(xlsx_path: String, csvs: Vec<PathBuf>, kwds: Option<&PyDict>) -> eyre::Result<String> {
    let options = kwds_to_options(kwds);
    let output = csvs_convert_rs::csvs_to_xlsx_with_options(xlsx_path.into(), csvs, options)?; 
    Ok(output.to_string())
}

#[pyfunction(kwds="**")]
fn datapackage_to_xlsx(xlsx_path: String, datapackage: String, kwds: Option<&PyDict>) -> eyre::Result<()> {
    let options = kwds_to_options(kwds);
    csvs_convert_rs::datapackage_to_xlsx_with_options(xlsx_path.into(), datapackage, options)?; 
    Ok(())
}

#[pyfunction(kwds="**")]
fn csvs_to_datapackage(mut datapackage: PathBuf, csvs: Vec<PathBuf>, kwds: Option<&PyDict>) -> eyre::Result<()> {
    let options = kwds_to_options(kwds);
    if datapackage.ends_with("datapackage.json") {
        datapackage.pop();
    }
    let describe_options = 
        csvs_convert_rs::DescribeOptions::builder().
        delimiter(options.delimiter).
        quote(options.quote).
        stats_csv(options.stats_csv).
        stats(options.stats).
        threads(options.threads).
        build();

    csvs_convert_rs::output_datapackage(csvs, datapackage, &describe_options)?;
    Ok(())
}


#[pymodule]
fn csvs_convert(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(merge_datapackage, m)?)?;
    m.add_function(wrap_pyfunction!(datapackage_to_sqlite, m)?)?;
    m.add_function(wrap_pyfunction!(csvs_to_sqlite, m)?)?;
    m.add_function(wrap_pyfunction!(datapackage_to_postgres, m)?)?;
    m.add_function(wrap_pyfunction!(csvs_to_postgres, m)?)?;
    m.add_function(wrap_pyfunction!(datapackage_to_parquet, m)?)?;
    m.add_function(wrap_pyfunction!(csvs_to_parquet, m)?)?;
    m.add_function(wrap_pyfunction!(datapackage_to_xlsx, m)?)?;
    m.add_function(wrap_pyfunction!(csvs_to_xlsx, m)?)?;
    m.add_function(wrap_pyfunction!(csvs_to_datapackage, m)?)?;
    Ok(())
}