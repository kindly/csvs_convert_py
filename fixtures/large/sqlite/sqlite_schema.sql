CREATE TABLE "daily_16"(
    "_link" TEXT,
    "time" NUMERIC,
    "city_id" NUMERIC,
    "city_name" TEXT,
    "city_country" TEXT,
    "city_coord_lon" NUMERIC,
    "city_coord_lat" NUMERIC);

CREATE TABLE "data"(
    "_link" TEXT,
    "_link_daily_16" TEXT,
    "dt" NUMERIC,
    "rain" NUMERIC,
    "pressure" NUMERIC,
    "humidity" NUMERIC,
    "uvi" NUMERIC,
    "speed" NUMERIC,
    "deg" NUMERIC,
    "clouds" NUMERIC,
    "temp_day" NUMERIC,
    "temp_min" NUMERIC,
    "temp_max" NUMERIC,
    "temp_night" NUMERIC,
    "temp_eve" NUMERIC,
    "temp_morn" NUMERIC,
    "snow" NUMERIC);

CREATE TABLE "data_weather"(
    "_link" TEXT,
    "_link_data" TEXT,
    "_link_daily_16" TEXT,
    "id" NUMERIC,
    "main" TEXT,
    "description" TEXT,
    "icon" TEXT);

