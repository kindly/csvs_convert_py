# Datapackage Convert

Conversions for tabular-data-packages:

* [Merge mulitple datapackages into one](merge.md)
* [To SQLite](sqlite.md)  
* [To Parquet](parquet.md)
* [To XLSX](xlsx.md)

All conversions aim to be memory efficiant and as fast they can be. This is the python library, providing bindings to the [rust library](https://github.com/kindly/datapackage_convert).


## Install

```bash
pip install datapackage_convert
```

## Usage

When refering to a datapackage you can either reference:

* A `datapackage.json` file.
* A datapackage directory containing a `datapackage.json` file. e.g.  `/a/datapackage/dir`
* A zip file containing a `datapackage.json` file. e.g. `my_datapackage.zip`



```{toctree}
:hidden:
changelog
merge
```
