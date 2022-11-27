# Options

## Delimiter 

`delimiter` will set the delimiter for all tables. If not given it will be detected and if not will default to `','`. 
Will use value in `datapackage.json` instead if exists. Can only be a single character.

Example:
```
from csvs_convert import csvs_to_sqlite
csvs_to_sqlite('output.db', ['data.csv', 'data2.csv'], delimiter=",")
```

## Quote

`quote` will set the quote character for all tables. By default will try and be detected and if not will default to `'"'`.
Will use value in `datapackage.json` instead if exists. Can only be a single character. Not for parquet conversion.

Example:
```
from csvs_convert import csvs_to_sqlite
csvs_to_sqlite(f'output.db', ['data.csv', 'data2.csv'], quote='"')
```

## Delete Input CSV

`delete_input_csv` deletes input CSVS after converting them.

Example:
```
from csvs_convert import csvs_to_sqlite
csvs_to_sqlite(f'output.db', ['data.csv', 'data2.csv'], delete_input_csv=True)
```

## Drop

`drop` when inserting into a database, if a table with that name exists, drop the table before loading the new data. SQLITE and POSTGRES only.

Example:
```
from csvs_convert import csvs_to_sqlite
csvs_to_sqlite(f'output.db', ['data.csv', 'data2.csv'], drop=True)
```

## Evolve

See [](evolve.md)
