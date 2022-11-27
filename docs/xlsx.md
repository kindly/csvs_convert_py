# To XLSX

Convert tabular-datapackage to XLSX file.

## Usage Example

```
from datapackage_convert import datapackage_to_xlsx

datapackage_to_xlsx('output.xlsx', ['path/to/datapackage'])
```

## Usage Example From Datapackage

```
from datapackage_convert import datapackage_to_xlsx

datapackage_to_xlsx('output.xlsx', '/path/to/datapackage')
```

## Notes and Caveats

Sheet names can not be longer than 31 characters in XLSX files.  The tool will truncate them if this is the case.  Updating the datapackage resource `title` fields and using the `use_titles` option can be used to customize sheet names.
