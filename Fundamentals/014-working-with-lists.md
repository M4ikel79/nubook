# Working with Lists

> **Tips**
> Lists are equivalent to the individual columns of tables. You can think of a list as essentially being a "one-column table" (with no column name). Thus, any command which operates on a column *also* operates on a list.

```nu
[bell book candle] | where ($it =~ 'b')
# => ╭───┬──────╮
# => │ 0 │ bell │
# => │ 1 │ book │
# => ╰───┴──────╯
```

## Creating lists

A list is an ordered collection of values. A list is created using square brackets surrounding values separated by spaces, linebreaks, and/or commas:

```nu
[foo bar baz]
# or
[foo, bar, baz]
```

> **Tips**
> Nushell lists are similar to JSON arrays.

## Updating lists

We can `insert` values into lists:

```nu
[1, 2, 3, 4] | insert 2 10
# => [1, 2, 10, 3, 4]
```

We can also use `update` to replace an element:

```nu
[1, 2, 3, 4] | update 1 10
# => [1, 10, 3, 4]
```

## Removing or Adding Items from List

We have `prepend` and `append` to insert at the beginning or end:

```nu
let colors = [yellow green]
let colors = ($colors | prepend red)
let colors = ($colors | append purple)
let colors = ($colors ++ ["blue"])
let colors = (["black"] ++ $colors)
$colors
# => [black red yellow green purple blue]
```

To remove items, use `skip`, `drop`, `first`, `last`:

```nu
let colors = [red yellow green purple]
let colors = ($colors | skip 1)      # [yellow green purple]
let colors = ($colors | drop 2)     # [yellow]
```

```nu
let colors = [red yellow green purple black magenta]
let colors = ($colors | last 3)
# => [purple black magenta]

let colors = [yellow green purple]
let colors = ($colors | first 2)
# => [yellow green]
```

### Using the Spread Operator

```nu
let x = [1 2]
[
  ...$x
  3
  ...(4..7 | take 2)
]
# => [1 2 3 4 5]
```

## Iterating over Lists

Use `each` with a block:

```nu
let names = [Mark Tami Amanda Jeremy]
$names | each { |elt| $"Hello, ($elt)!" }
# Outputs "Hello, Mark!" and three more similar lines.

$names | enumerate | each { |elt| $"($elt.index + 1) - ($elt.item)" }
# Outputs "1 - Mark", "2 - Tami", etc.
```

The `where` command creates a subset of a list:

```nu
let colors = [red orange yellow green blue purple]
$colors | where ($it | str ends-with 'e')
# => [orange blue purple]

let scores = [7 10 8 6 7]
$scores | where $it > 7
# => [10 8]
```

The `reduce` command computes a single value from a list:

```nu
let scores = [3 8 4]
$"total = ($scores | reduce { |elt, acc| $acc + $elt })"
# => total = 15

$"product = ($scores | reduce --fold 1 { |elt, acc| $acc * $elt })"
# => product = 96
```

## Accessing the List

To access a list item at a given index:

```nu
let names = [Mark Tami Amanda Jeremy]
$names.1 # gives Tami
```

Using `get` command:

```nu
let names = [Mark Tami Amanda Jeremy]
let index = 1
$names | get $index # gives Tami
```

Using `length`:

```nu
[red green blue] | length
# => 3
```

Using `is-empty`:

```nu
let colors = [red green blue]
$colors | is-empty # false

let colors = []
$colors | is-empty # true
```

Using `in` and `not-in`:

```nu
let colors = [red green blue]
'blue' in $colors # true
'yellow' in $colors # false
```

Using `any`:

```nu
let colors = [red green blue]
$colors | any {|elt| $elt | str ends-with "e" } # true
$colors | any {|elt| ($elt | str length) < 3 } # false
```

Using `all`:

```nu
let colors = [red green blue]
$colors | all {|elt| $elt | str ends-with "e" } # false
$colors | all {|elt| ($elt | str length) >= 3 } # true
```

## Converting the List

The `flatten` command creates a new list from nested lists:

```nu
[1 [2 3] 4 [5 6]] | flatten
# => [1 2 3 4 5 6]
```

The `wrap` command converts a list to a table:

```nu
let zones = [UTC CET Europe/Moscow Asia/Yekaterinburg]
$zones | wrap 'Zone' | upsert Time {|row| (date now | date to-timezone $row.Zone | format date '%Y.%m.%d %H:%M')}
```
