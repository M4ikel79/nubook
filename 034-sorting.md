# Sorting

Nushell offers many ways of sorting data. Let's explore the options.

## Basic Sorting

### Lists

```nu
[9 3 8 1 4 6] | sort
# => [1, 3, 4, 6, 8, 9]
```

With mixed types (numbers and strings):
```nu
["hello" 4 9 2 1 "foobar" 8 6] | sort
# => [1, 2, 4, 6, 8, 9, foobar, hello]
```

Numbers are sorted first, then strings. Nushell's sort is **stable** - equal values retain their original ordering.

### Records

Sort by key:
```nu
{x: 123, a: "hello!", foo: "bar"} | sort
# => {a: "hello!", foo: "bar", x: 123}
```

Sort by value:
```nu
{x: 123, a: "hello!", foo: "bar"} | sort -v
# => {x: 123, foo: "bar", a: "hello!"}
```

### Tables

Rows are sorted by comparing columns in order:
```nu
let items = [
    {id: 100, quantity: 10, price: 5}
    {id: 100, quantity: 5, price: 8}
    {id: 100, quantity: 5, price: 1}
]
$items | sort
# Sorts by id, then quantity, then price
```

## Sorting Structured Data

### Cell Path

Sort by column using `sort-by`:
```nu
ls | sort-by size
ls | sort-by size modified  # multiple columns (tiebreaker)
```

Sort nested data:
```nu
let cities = [
    {name: 'New York', info: { established: 1624, population: 18_819_000 }}
    {name: 'Kyoto', info: { established: 794, population: 37_468_000 }}
]
$cities | sort-by info.established
```

### Key Closure

Sort using a closure to transform values:
```nu
let assignments = [
    {name: 'Homework 1', grades: [97, 89, 86, 92, 89]}
    {name: 'Exam 1', grades: [78, 88, 78, 53, 90]}
]
$assignments | sort-by { get grades | math avg }
```

Use parameter:
```nu
let weight = {alpha: 10, beta: 5, gamma: 3}
[alpha, gamma, beta] | sort-by {|val| $weight | get $val }
```

### Custom Sort Order

Use `-c` flag with closure for custom ordering:
```nu
ls | sort-by -c {|a, b| $a.size < $b.size }
```

Complex example - sort by priority, with zero work time first:
```nu
let queue = [
    {task: 139, work_time: 0, priority: 1}
    {task: 52, work_time: 355, priority: 8}
]
let my_sort = {|a, b|
    match [$a.work_time, $b.work_time] {
        [0, 0] => ($a.priority > $b.priority)
        [0, _] => true
        [_, 0] => false
        _ => ($a.priority > $b.priority)
    }
}
$queue | sort-by -c $my_sort
```

## Special Sorts

### Case Insensitive

```nu
[foo FOO bar BAR] | sort -i
# => [bar, BAR, foo, FOO]
```

### Natural Sort

Sort strings containing numbers naturally:
```nu
[10, 9, foo123, foo20] | sort
# => [10, 9, bar123, bar20, foo123, foo20]

[10, 9, foo123, foo20] | sort -n
# => [9, 10, bar20, bar123, foo20, foo123]
```

Mix numbers and numeric strings:
```nu
[4, "6.2", 1, "10"] | sort -n
# => [1, 2, "3", 4, 5.5, "6.2", 7, 8.1, "9", 10]
```

### Mixed Types

In sorted order, values of the same type appear together:
- Integers and floats are intermixed
- Strings and globs are intermixed
- null values always sorted to the end

### Strict Sort

Use custom sort to reject incompatible types:
```nu
["hello" 4 9] | sort-by -c {|a, b| $a < $b | default ($a != null) }
# => Error: type_mismatch
```
