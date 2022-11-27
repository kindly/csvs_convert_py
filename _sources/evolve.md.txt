# Evolve 

For postgres and sqlite you evolve the existing tables if the schema of the new data is different. This is useful if you have new data that comes in over time that has a risk of having additional field.

Evolving follows the following rules:

- If the new data contains a table that does not exist in the database it will created.
- If the table already exists but the new data has extra fields, the table is altered to add the new fields.
- If table exists but fields that are in the database are not in the new data, they will result in nulls in the database when the new data is inserted. 
- If table exists and contains the same field name as the new data but the data types of the fields conflict:
  - For postgres the field is altered to being a `text` field so that both new and old data can exist (all types can be coerced to text).
  - For sqlite, as you can not alter existing types, the original type is kept. This will mean the data insertion will still work as sqlite treats any field as if it is text.  

**Warning: this could mean you modify existing data.**
