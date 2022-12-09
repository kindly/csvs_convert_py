# Change Log
All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [0.7.1] - 2022-12-07

### New

- `stats` and `stats_csv` options to make stats about the data.
- `csvs_to_*` commands not return the datapackage as a python dict insead of None.

## [0.7.0] - 2022-12-07

### New

- Changed name to `csvs_convert`
- All conversions now accept a list of `CSV` files.
- Type guessing for `CSV` files generating a `datapackage.json` file.

## [0.5.2] - 2022-07-27

### New

- environment var postgres support

## [0.5.0] - 2022-07-20

### New

- postgres support 

## [0.4.0] - 2022-05-14

### New

- XLSX support 
- Allow options too be passed to rust library
- Docs and tests in python libary