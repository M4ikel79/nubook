# Working with Tables

One of the common ways of seeing data in Nu is through a table. Nu comes with a number of commands for working with tables.

## Sorting the Data

Sort by a column:

```nu
ls | sort-by size
ls | sort-by name
```

## Selecting the Data you Want

Select specific columns:

```nu
ls | select name size
```

Get first/skip rows:

```nu
ls | sort-by size | first 5
ls | sort-by size | first 5 | skip 2
```

Get a specific row by index:

```nu
ls | sort-by name | select 5
```

## Getting Data out of a Table

Get values from a column:

```nu
ls | get name
```

- `select` - creates a new table with only the columns specified
- `get` - returns the values inside the column as a list

## Changing Data in a Table

### Concatenating Tables

```nu
let first = [[a b]; [1 2]]
let second = [[a b]; [3 4]]
$first | append $second
# => ╭───┬───┬───╮
# => │ a │ b │
# => ├───┼───┼───┤
# => │ 1 │ 2 │
# => │ 3 │ 4 │
# => ╰───┴───┴───╯
```

Or use `++` operator:

```nu
$first ++ $second
```

### Merging Tables

```nu
let first = [[a b]; [1 2]]
let second = [[c d]; [3 4]]
$first | merge $second
# => ╭───┬───┬───┬───╮
# => │ a │ b │ c │ d │
# => ├───┼───┼───┼───┤
# => │ 1 │ 2 │ 3 │ 4 │
# => ╰───┴───┴───┴───╯
```

### Adding a new Column

```nu
open rustfmt.toml | insert next_edition 2021
```

### Updating a Column

```nu
open rustfmt.toml | update edition 2021
```

Use `upsert` to insert or update depending on existence.

### Moving Columns

```nu
ls | move name --after size
```

### Renaming Columns

```nu
ls | rename filename filetype filesize date
```

### Rejecting/Deleting Columns

```nu
ls -l / | reject readonly num_links inode created accessed modified
```

## The # Index Column

Tables have a `#` column that can operate in two modes:
1. Internal # (default) - 0-based consecutive index
2. "Index"-renamed # - when a column named "index" is created

Convert # to index:

```nu
ls | enumerate | flatten
```

## The table command

The `table` command renders structured data:

```nu
[items] | table | ansi strip
```

Options:
- `-e` to expand collapsed data
- `-i false` to hide the index column
- `-a 5` to abbreviate to first and last 5 entries
