# To Postgres

A database connection string is needed, using the [format expressed here](https://docs.rs/postgres/latest/postgres/config/struct.Config.html#examples-1).  If you want the connection string to be got from an environment variable then use the string `env` (for default `DATABASE_URL` enviroment variable) or `env=MY_ENV_VAR` (for `MY_ENV_VAR` environment variable).

If a table does not exist in the database it will be created.

If the table already exists in the database, rows will be appended to the existing table. There will be errors if the field definitions of the data do not match the field definitions in the database unless `evolve=True` is set.  Use `drop=True` if instead you want delete the table if it exists.


## Usage Example
```
from csvs_convert import csvs_to_postgres

csvs_to_postgres('postgresql://user:postgres@localhost/db', ['/path/to/file.csv'], drop=True, schema='myschema')
```


## Usage Example from datapackage
```
from csvs_convert import datapackage_to_postgres

datapackage_to_postgres('postgresql://user:postgres@localhost/db', '/path/to/datapackage', drop=True, schema='myschema')
```

## Usage Example From Enviroment Variable

Uses environment variable DATABASE_URL.
```
csvs_to_postgres('env', ['/path/to/file.csv'])
```

Uses environment variable MY_ENV_VAR.
```
csvs_to_postgres('env=MY_ENV_VAR', ['/path/to/file.csv'])
```

## Evolve

See [](evolve.md)

```
csvs_to_postgres('postgresql://user:postgres@localhost/db', ['/path/to/file.csv'], evolve=True)
```

## Schema

This will put the tables in the datapackage into a postgres schema. If the schema does not exist it will be created. 

Example

```
datapackage_to_postgres('postgresql://user:postgres@localhost/db', '/path/to/datapackage', schema='myschema')
```

### Drop

This will drop the tables in the database before recreating them.  WARNING: data may be lost if you select this option. 

Example

```
csvs_to_postgres('postgresql://user:postgres@localhost/db', ['/path/to/data.csv'], drop=True)
```