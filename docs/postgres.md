# To Postgres

Convert tabular-datapackage to postgres database.

## Usage Example

```
from datapackage_convert import datapackage_to_postgres

datapackage_to_postgres('postgresql://user:postgres@localhost/db', '/path/to/datapackage', drop=True, schema='myschema')
```

## Usage Example From Enviroment Variable

Uses environment variable DATABASE_URL.

```
datapackage_to_postgres('env', '/path/to/datapackage')
```

Uses environment variable MY_ENV_VAR.

```
datapackage_to_postgres('env=MY_ENV_VAR', '/path/to/datapackage')
```

## Options

First argument is a database connection string using the [format expressed here](https://docs.rs/postgres/latest/postgres/config/struct.Config.html#examples-1).  If you want the connection string to be got from an environment variable then use the string `env` (for default `DATABASE_URL` enviroment variable) or `env=MY_ENV_VAR` (for `MY_ENV_VAR` environment variable).

Second argument is the path to datapackage.

If a table does not exist in the database they will be created.

If the table already exists in the database, rows will be appended to the existing table. There will be errors if the field definitions of the datapackage do not match the field definitions in the database.

### schema

This will put the tables in the datapackage into a postgres schema. If the schema does not exist it will be created. 

Example

```
datapackage_to_postgres(f'postgresql://user:postgres@localhost/db', '/path/to/datapackage', schema='myschema')
```

### drop

This will drop the tables in the database expressed in the datapackage before recreating them.  WARNING: data may be lost if you select this option. 


Example

```
datapackage_to_postgres(f'postgresql://user:postgres@localhost/db', '/path/to/datapackage', drop=True)
```



### delete_input_csv

This will delete the input csvs from the orginal datapackage.  This is useful if the files are large and you just want to keep the sqlite databasae and not the CSV files.

Example

```
datapackage_to_postgres(f'postgresql://user:postgres@localhost/db', '/path/to/datapackage', delete_input_csv=True)
```