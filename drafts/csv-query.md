# CSV Query Compiler

Compile a tiny query language for CSV files (with string or floating-point fields) into specialized row processors.

- Select named columns
- Calculate values for new columns
- Filter with comparisons and boolean operators
- Aggregate ops (count, sum)


## First pass

Load a headered CSV file and interpret expressions against one row at a time.


## Compiler extension

Parse queries into an expression tree and lower them to bytecode, generated C, or native code.
Specialize field lookups using column indices and short-circuit boolean conditions.
Optionally fuse filter, projection, and aggregation into a single loop.


## Evidence of success

Run queries over various sample datasets (TODO: flights, weather, synthetic sales?).
Print aggregate results.
Compare interpreted and compiled throughput in rows per second.


## Optional challenges

- Joins
- Sorting
- Group-by
- Additional cell datatypes: integer, time, date, etc.
