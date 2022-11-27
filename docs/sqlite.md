# To Sqlite

Convert csvs to sqlite.

## From CSV files usage example

First argument is name of database file.  If database already exists new tables will created with in it.  Second is path to datapackage.

```
from csvs_convert import csvs_to_sqlite

csvs_to_sqlite(f'output.db', ['data.csv', 'data2.csv'])
```

## From datapackage usage example

```
from csvs_convert import datapackage_to_sqlite

datapackage_to_sqlite('output.db', '/path/to/datapackage')
```

## Evolve

See [](evolve.md)

```
csvs_to_sqlite('sqlite.db', ['/path/to/file.csv'], evolve=True)
```
