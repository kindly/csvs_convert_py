# Datapackage Convert

Conversions from tabular-data-packages. Currently:

* Merge mulitple datapackages into one.
* To SQLite  
* To Parquet 

## Install

```
pip install datapackage_convert
```

## Usage

When refering to a datapackage you can either reference:

* A `datapackage.json` file.
* A datapackage directory containing a `datapackage.json` file. e.g.  `/a/datapackage/dir`
* A zip file containing a `datapackage.json` file. e.g. `my_datapackage.zip`

### Merge datapackage

This will merge datapckages into a single one.

```
from datapackage_convert import merge_datapackage

merge_datapackage('output_datapackage', ['datapackage1', 'datapackage2'])
```

### To SQLite

Will convert to sqlite file. Adding foreign key relationships and indexes for them.

```
from datapackage_convert import datapackage_to_sqlite

datapackage_to_sqlite('mydatabase.db', 'my_datapackage')
```

### To parquet

Will convert to snappy compressed parquet files. 

```
from datapackage_convert import datapackage_to_parquet

datapackage_to_parquet('output_directory', 'my_datapackage')
```

