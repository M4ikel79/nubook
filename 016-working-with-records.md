# Working with Records

> **Tips**
> Records are roughly equivalent to the individual rows of a table. You can think of a record as essentially being a "one-row table".

```nu
let my_record = { name: "Sam", age: 30 }
$my_record | update age { $in + 1 }
# => {name: Sam, age: 31}
```

Note that the `my_record` variable is immutable. The updated record is printed but `$my_record.age` is still `30`.

## Creating records

A record is a collection of zero or more key-value pair mappings:

```nu
{ "apples": 543, "bananas": 411, "oranges": 0 }
# => ╭─────────┬─────╮
# => │ apples  │ 543 │
# => │ bananas │ 411 │
# => │ oranges │ 0   │
# => ╰─────────┴─────╯
```

To display a record horizontally:

```nu
{ name: "Sam", rank: 10 } | to nuon
# => {name: Sam, rank: 10}
```

## Updating Records

Insert values:

```nu
{ "apples": 543, "bananas": 411, "oranges": 0 } | insert pears { 21 }
# => {apples: 543, bananas: 411, oranges: 0, pears: 21}
```

Update values:

```nu
{ "apples": 543, "bananas": 411, "oranges": 0 } | update oranges { 100 }
# => {apples: 543, bananas: 411, oranges: 100}
```

Merge records:

```nu
let first_record = { name: "Sam", rank: 10 }
$first_record | merge { title: "Mayor" }
# => {name: Sam, rank: 10, title: Mayor}
```

Or use the spread operator:

```nu
{ ...$first_record, title: "Mayor" }
```

## Iterating over a Record

Use `items` to iterate over key-value pairs:

```nu
{ "apples": 543, "bananas": 411, "oranges": 0 } | items {|fruit, count| $"We have ($fruit) ($count)" }
```

Or transpose and iterate:

```nu
{ "apples": 543, "bananas": 411, "oranges": 0 } | transpose fruit count | each {|f| $"We have ($f.count) ($f.fruit)" }
```

## Accessing Record Values

Use dot notation or cell-paths:

```nu
let my_record = { name: "Sam", age: 30 }
$my_record.name   # "Sam"
$my_record.age    # 30
```
