# To XLSX

Convert tabular-datapackage to XLSX file.

## Usage Example

```
from datapackage_convert import datapackage_to_xlsx

datapackage_to_xlsx(f'output.xlsx', '/path/to/datapackage')
```

## Options

First argument is name of output xlsx file.  If file already exists it will be deleted.  Second is path to datapackage.

### use_titles

This will use resource `title` (instead of resource name) from the datapackage to determine sheet name.

Example

```
datapackage_to_xlsx(f'output.db', '/path/to/datapackage', use_titles=True)
```

### delete_input_csv

This will delete the input csvs from the orginal datapackage.  This is useful if the files are large and you just want to keep the XLSX file and not the CSV files.

Example

```
datapackage_to_xlsx(f'output.db', '/path/to/datapackage', delete_input_csv=True)
```

## Notes and Caveats

Sheet names can not be longer than 31 characters in XLSX files.  The tool will truncate them if this is the case.  Updating the datapackage resource `title` fields and using the `use_titles` option can be used to customise sheet names.
