from .csvs_convert import merge_datapackage
from .csvs_convert import datapackage_to_sqlite
from .csvs_convert import datapackage_to_postgres
from .csvs_convert import datapackage_to_parquet
from .csvs_convert import datapackage_to_xlsx
from .csvs_convert import csvs_to_datapackage

import csvs_convert.csvs_convert as csvs_convert_rs

import json

def csvs_to_sqlite(*args, **kw):
    datapackage_string = csvs_convert_rs.csvs_to_sqlite(*args, **kw)
    return json.loads(datapackage_string)

def csvs_to_postgres(*args, **kw):
    datapackage_string = csvs_convert_rs.csvs_to_postgres(*args, **kw)
    return json.loads(datapackage_string)

def csvs_to_xlsx(*args, **kw):
    datapackage_string = csvs_convert_rs.csvs_to_xlsx(*args, **kw)
    return json.loads(datapackage_string)

def csvs_to_parquet(*args, **kw):
    datapackage_string = csvs_convert_rs.csvs_to_parquet(*args, **kw)
    return json.loads(datapackage_string)