# Navigating and Accessing Structured Data

Given Nushell's strong support for structured data, some of the more common tasks involve navigating and accessing that data.

## Background and Definitions

- **List:** A series of zero or more values of any type
- **Record:** Zero or more pairs of named keys and their corresponding values
- **Nested Data:** Values can be basic types or structured data themselves (lists, tables, records)

> **Tips**
> Because a table is a list of records, any command or syntax that works on a list will also work on a table.

## Cell-paths

A cell-path is the primary way to access values inside structured data. It uses column names and row numbers separated by dots.

### Records

```nu
let my_record = { a: 5, b: 42 }
$my_record.b + 5
# => 47
```

### Lists

List indices are 0-based:

```nu
let scoobies_list = [Velma Fred Daphne Shaggy Scooby]
$scoobies_list.2
# => Daphne
```

### Tables

- Access a column by name: `$table.column_name`
- Access a row by index: `$table.0`
- Access a cell: `$table.column_name.row_index`

```nu
let data = [
    [date, temps, condition];
    [2022-02-01, [38.24, 38.50], 'sunny'],
    [2022-02-02, [35.24, 35.94], 'sunny']
]

$data.condition    # Access column -> list
$data.1           # Access row -> record
$data.condition.1 # Access cell -> value
```

### Nested Data

```nu
$data.temps.2.1  # Third row, second temp reading
```

## Using `get` and `select`

- `get` - returns the **value** indicated by the cell-path
- `select` - returns the specified **data structure** itself

```nu
$data | get 1         # Returns the record at index 1
$data | select 1      # Returns a single-row table

$data | select date condition 0 1  # Multiple columns and rows
```

## Key/Column names with spaces

Quote keys with spaces:

```nu
let record_example = { "key x": 12, "key y": 4 }
$record_example."key x"
# => 12
```

## Handling Missing Data

### The Optional Operator

Add `?` to suppress errors for missing data:

```nu
let cp: cell-path = $.temps?.1
$data | reject temps | get $cp
# Returns null if temps column doesn't exist
```

### Default values

```nu
let missing_value = [{a:1 b:2} {b:1}]
$missing_value | default 'n/a' a
# => {a: n/a, b: 1}
```

## Other commands for accessing structured data

- `reject` - opposite of select, removing rows/columns
- `slice` - select rows using a range type
