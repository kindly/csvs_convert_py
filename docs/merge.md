# Merge

Merge two or more tabular datapackages.  If the paths within the datapackage match append the CSV files together, including fields from the matching CSV files.

## Usage Example

This is a simple example of merging two datapackages where the second file adds a field and a new file.

```
from datapackage_convert import merge_datapackage

merge_datapackage(f'/tmp/output', ['base_datapackage', 'add_field_and_file'])
```

### base_datapackage

The base_datapackage directory in the above example looks like:

```
├── base_datapackage
│   ├── csv
│   │   └── games.csv
│   └── datapackage.json
```
And games.csv looks like:

|id |title     |
|---|----------|
|1  |game1_base|
|2  |game2_base|

### add_field_and_file datapackage

We are merging to following datapackage

```
├── add_field
│   ├── csv
│   │   ├── games.csv
│   │   └── apps.csv
│   └── datapackage.json
```

And games.csv looks like:

|id |title2    |
|---|----------|
|3  |game1_add_field|

### result datapackage

After merging the output folder looks like:

```
├── add_field
│   ├── csv
│   │   ├── games.csv
│   │   └── apps.csv
│   └── datapackage.json
```

The `app.csv` is the same as in the `add_field_and_file` as it only appears in that datapackage. The `games.csv` looks like:


|id |title     |title2         |
|---|----------|---------------|
|1  |game1_base|               |
|2  |game2_base|               |
|3  |          |game1_add_field|

As you can see the `games.csv` files have been merged and the fields from both exist. `title` only exists in the first datapackage and `title2` only exists in the second. Once merged both columns exist and the data is left blank when they do not exist.

## Options

### delete_input_csv

This will delete the input csvs from the orginal datapackages once merged.  This is useful if the files are large and you just want to keep the merged datapackages.

Example

```
merge_datapackage(f'/tmp/output', ['base_datapackage', 'add_field_and_file'], delete_input_csv=True)
```

## Notes and Caveats

A new `datapackage.json` is created after merging.  

If the two or more files are merged and thier fields match but the fields are of different types then the new `datapackage.json` will just fall back to saying the field is a `string`.