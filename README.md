# CSVS Convert

Converts CSV files into XLSX/SQLITE/POSTGRESQL/PARQUET fast.  

## Install

```bash
pip install csvs_convert
```

## Docs

[Full Documentaion](http://datapackage_convert.opendata.coop)

## Aims

* Thorough type guessing of CSV columns, so there is no need to configure types of each field. Scans whole file first to make sure all types in a column are consistent. Can detect over 30 date/time formats as well as JSON data.
* Quick conversions/type guessing (uses rust underneath). Uses fast methods specific for each output format:
    * `copy` for postgres
    * Prepared statements for sqlite using c API.
    * Arrow reader for parquet
    * Write only mode for libxlsxwriter
* Tries to limit errors when inserting data into database by resorting to "text" if type guessing can't determine a more specific type.
* When inserting into existing databases automatically migrate schema of target to allow for new data (`evolve` option).
* Memory efficient. All csvs and outputs are streamed so all conversions should take up very little memory.
* Gather stats and information about CSV files into datapacakge.json file which can use it for customizing conversion.

## Drawbacks

* CSV files currently need header rows.
* Whole file needs to be on disk as whole CSV is analyzed therefore files are read twice.

## Conversion Docs

This is the python library, providing bindings to the [rust library](https://github.com/kindly/csvs_convert).

[**Contribute on github**](https://github.com/kindly/csvs_convert_py)


## Usage From CSV files.

```
import csvs_convert

#sqlite
csvs_convert.csvs_to_sqlite("sqlite.db", ["file.csv"])
#postgres
csvs_convert.csvs_to_postgres("postgresql://user:postgres@localhost/db", ["file.csv"])
#parquet
csvs_convert.csvs_to_parquet("output", ["file.csv"])
#xlsx
csvs_convert.csvs_to_xlsx("output.xlsx", ["sqlite.db"])
```

## Usage from datapackage

A datapackage is a file that contains metadata about the tables its specification is described [here](https://datahub.io/docs/data-packages/tabular).

To generate `datapackage.json` file you can use:

```
csvs_convert.csvs_to_datapackage('path/to/datapackage.json', ["fixtures/large/csv/data.csv"])
```

Other tools can also generate these files.

You can use this file and alter it as needed. Mostly it is useful if you want to use the same schema across multiple files, as it will save time not having to do the type guessing for every file.

When referring to a datapackage you can either reference:

* A `datapackage.json` file.
* A datapackage directory containing a `datapackage.json` file. e.g.  `/a/datapackage/dir`
* A zip file containing a `datapackage.json` file. e.g. `my_datapackage.zip`

### Examples:
```
import csvs_convert

#sqlite
csvs_convert.datapackage_to_sqlite("sqlite.db", "path/to/datapackage.json")
#postgres
csvs_convert.datapackage_to_postgres("postgresql://user:postgres@localhost/db", "path/to/datapackage.json")
#parquet
csvs_convert.datapackage_to_parquet("path/to/directory", ["sqlite.db"])
#xlsx
csvs_convert.datapackage_to_xlsx("output.xlsx", "path/to/datapackage.json")
```

