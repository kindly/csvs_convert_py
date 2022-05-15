# To Sqlite

Convert tabular-datapackage to sqlite.

## Usage Example

```
from datapackage_convert import datapackage_to_sqlite

datapackage_to_sqlite(f'output.db', '/path/to/datapackage')
```

## Options

First argument is name of database file.  If database already exists new tables will created with in it.  Second is path to datapackage.

### delete_input_csv

This will delete the input csvs from the orginal datapackage.  This is useful if the files are large and you just want to keep the sqlite databasae and not the CSV files.

Example

```
datapackage_to_sqlite(f'output.db', '/path/to/datapackage', delete_input_csv=True)
```
