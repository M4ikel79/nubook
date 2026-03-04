# Types of Data

Traditional Unix shell commands communicate with each other using strings of text -- One command writes text to standard output (often abbreviated `stdout`) and the other reads text from standard input (or `stdin`). This allows multiple commands to be combined together to communicate through what is called a "pipeline".

Nushell embraces this approach and expands it to include other types of data in addition to strings.

Like many programming languages, Nu models data using a set of simple, structured data types. Simple data types include integers, floats, strings, and booleans. There are also special types for dates, file sizes, and time durations.

The `describe` command returns the type of a data value:

```nu
42 | describe
# => int
```

## Types at a Glance

| Type | Example |
|------|---------|
| Integers | `-65535` |
| Floats (decimals) | `9.9999`, `Infinity` |
| Strings | `"hole 18"`, `'hole 18'`, `` `hole 18` ``, `hole18`, `r#'hole18'#` |
| Booleans | `true` |
| Dates | `2000-01-01` |
| Durations | `2min + 12sec` |
| File-sizes | `64mb` |
| Ranges | `0..4`, `0..<5`, `0..`, `..4` |
| Binary | `0x[FE FF]` |
| Lists | `[0 1 'two' 3]` |
| Records | `{name:"Nushell", lang: "Rust"}` |
| Tables | `[{x:12, y:15}, {x:8, y:9}]`, `[[x, y]; [12, 15], [8, 9]]` |
| Closures | `{|e| $e + 1 | into string }`, `{ $in.name.0 | path exists }` |
| Cell-paths | `$.name.0` |
| Blocks | `if true { print "hello!" }`, `loop { print "press ctrl-c to exit" }` |
| Null (Nothing) | `null` |
| Any | `let p: any = 5` |

## Basic Data Types

### Integers

Numbers without a fractional component (positive, negative, and 0).

```nu
10 / 2
# => 5
5 | describe
# => int
```

### Floats/Decimals

Numbers with some fractional component.

```nu
2.5 / 5.0
# => 0.5
```

### Text/Strings

A series of characters that represents text.

```nu
let audience: string = "World"
$"Hello, ($audience)"
# => Hello, World
```

### Booleans

True or False value.

```nu
let mybool: bool = (2 > 1)
$mybool
# => true
```

### Dates

Represents a specific point in time using international standard date-time descriptors.

```nu
date now
# => Mon, 12 Aug 2024 13:59:22 -0400 (now)
```

### Durations

Represent a unit of a passage of time.

```nu
3.14day
# => 3day 3hr 21min
30day / 1sec
# => 2592000
```

### File sizes

Specialized numeric type to represent the size of files or a number of bytes.

```nu
0.5kB
# => 500 B
1GiB / 1B
# => 1073741824
```

### Ranges

Describes a range of values from a starting value to an ending value, with an optional stride.

```nu
1..5
# => ╭───┬───╮
# => │ 0 │ 1 │
# => │ 1 │ 2 │
# => │ 2 │ 3 │
# => │ 3 │ 4 │
# => │ 4 │ 5 │
# => ╰───┴───╯
```

### Cell Paths

An expression that is used to navigate to an inner value in a structured value.

```nu
let cp = $.2
[ foo bar goo glue ] | get $cp
# => goo
```

### Closures

An anonymous function, often called a lambda function, which accepts parameters and *closes over* variables from outside its scope.

```nu
let compare_closure = {|a| $a > 5 }
let original_list = [ 40 -4 0 8 12 16 -16 ]
$original_list | where $compare_closure
# => ╭───┬────╮
# => │ 0 │ 40 │
# => │ 1 │  8 │
# => │ 2 │ 12 │
# => │ 3 │ 16 │
# => ╰───┴────╯
```

### Binary data

Represents binary data.

```nu
open nushell_logo.jpg
| into binary
| first 2
| $in == 0x[ff d8]
# => true
```

## Structured Data Types

Nushell includes a collection of structured data types that can contain the primitive types above.

### Lists

Ordered sequence of zero or more values of any type.

```nu
[Sam Fred George]
# => ╭───┬────────╮
# => │ 0 │ Sam    │
# => │ 1 │ Fred   │
# => │ 2 │ George │
# => ╰───┴────────╯
```

### Records

Holds key-value pairs which associate string keys with various data values.

```nu
let my_record = {
  name: "Kylian"
  rank: 99
}
$my_record
# => ╭───────┬────────────╮
# => │ name  │ Kylian     │
# => │ rank  │ 99         │
# => ╰───────┴────────────╯

$my_record | get name
# => Kylian
```

### Tables

A two-dimensional container with both columns and rows where each cell can hold any basic or structured data type.

> **Tips**
> Internally, tables are simply **lists of records**.

## Other Data Types

### Any

When used in a type annotation or signature, matches any type.

### Blocks

A syntactic form used by some Nushell keywords (e.g., `if` and `for`).

```nu
if true { print "It's true" }
```

### Nothing (Null)

The `nothing` type is to be used to represent the absence of another value.

```nu
let simple_record = { a: 5, b: 10 }
$simple_record.a?
# => 5
$simple_record.c?
# => Nothing is output
$simple_record.c? == null
# => true
```
