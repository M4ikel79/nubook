# DataFrames

**Important:** This feature requires the Polars plugin. See the Plugins Chapter to learn how to install it.

To test that the plugin is properly installed, run `help polars`.

## Overview

Nushell makes working with data its main priority. Lists and Tables help cycle through values for multiple operations. However, certain operations (like group-by or join with large datasets) can be costly with row-based data layout.

For this reason, the DataFrame structure was introduced. A DataFrame stores data in columnar format using Apache Arrow and uses Polars for fast columnar operations.

## Benchmark Comparisons

DataFrames can be significantly faster than native Nushell commands and even Python Pandas.

### Example: Group-by Operation

Using native Nushell commands: ~3.3 seconds
Using Python Pandas: ~1.3 seconds
Using Nushell DataFrames (Polars): ~135ms

The Polars plugin managed to finish 10 times faster than Pandas!

## Working with DataFrames

### Opening Files

```nu
let df = polars open --eager test_small.csv
```

`polars open` can read: csv, tsv, parquet, json(l), arrow, and avro.

### Viewing DataFrames

```nu
$df
polars store-ls | select key type columns rows estimated_size
```

### Basic Aggregations

```nu
$df | polars sum | polars collect
$df | polars sum | polars select int_1 int_2 | polars collect
```

### Joining DataFrames

```nu
$df_1 | polars join $df_2 int_1 int_1
$df_1 | polars join $df_2 [int_1 first] [int_1 first]  # multiple columns
```

### Group-by

```nu
let group = $df | polars group-by first
$group | polars agg (polars col int_1 | polars sum)
```

Multiple aggregations:
```nu
$group | polars agg [
    (polars col int_1 | polars n-unique)
    (polars col int_2 | polars min)
    (polars col float_1 | polars sum)
] | polars sort-by first
```

## Creating Dataframes

From Nushell primitives:
```nu
let df = [[a b]; [1 2] [3 4] [5 6]] | polars into-df
```

Add columns:
```nu
let df2 = $df | polars with-column $df.a --name a2
```

## Working with Series

A Series is the building block of a DataFrame - each column.

```nu
let series = [9 8 4] | polars into-df
let doubled = $series * 3 + 10
let renamed = $doubled | polars rename "0" memorable
```

Operations with two Series:
```nu
$series - $doubled
```

Add to DataFrame:
```nu
let df_new = $df | polars with-column $series --name new_col
```

## Series and Masks

Boolean masks for filtering:
```nu
let mask = $series == 8
$df | polars filter-with $mask
```

AND/OR operations:
```nu
$mask_0 and $mask_1
$mask_0 or $mask_1
```

## Series as Indices

Using Series as indices to select rows:
```nu
let indices = [1 4 6] | polars into-df
$df | polars take $indices
```

Using arg-sort:
```nu
let sorted_indices = $df | polars get word | polars arg-sort
$df | polars take $sorted_indices
```

## Unique Values

```nu
$df | polars get first | polars value-counts
```

## Lazy Dataframes

Dataframes can be lazy for optimized query planning:
```nu
let lazy_df = polars open test.csv
$lazy_df | polars filter (polars col year > 2010) | polars collect
```

## Dataframe Commands

List all available commands:
```nu
scope commands | where category =~ dataframe
```
