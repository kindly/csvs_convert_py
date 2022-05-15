# Datapackage Convert

Conversions from tabular-data-packages. Currently:

* Merge mulitple datapackages into one.
* To SQLite  
* To Parquet 
* To XLSX

All conversions aim to be memory efficiant and as fast they can be. This is the python library, providing bindings to the [rust library](https://github.com/kindly/datapackage_convert).

## Install

```
pip install datapackage-convert
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

### To xlsx

Will convert to XLSX files. 

```
from datapackage_convert import datapackage_to_xlsx

datapackage_to_xlsx('output.xlsx', 'my_datapackage')
```