# To Parquet

Convert tabular-datapackage to parquet files.

## Usage Example

```
from datapackage_convert import datapackage_to_parquet

datapackage_to_parquet(f'/path/to/output_dir', '/path/to/datapackage')
```

## Options

First argument is directory, will be created if it does not exist. Second is path to datapackage.

### delete_input_csv

This will delete the input csvs from the orginal datapackage.  This is useful if the files are large and you just want to keep the parquet files and not the CSV files.

Example:

```
datapackage_to_parquet(f'/path/to/output_dir', '/path/to/datapackage', delete_input_csv=True)
```
