# Variables

Nushell values can be assigned to named variables using the `let`, `const`, or `mut` keywords. After creating a variable, we can refer to it using `$` followed by its name.

## Types of Variables

### Immutable Variables

An immutable variable cannot change its value after declaration:

```nu
let val = 42
$val
# => 42
```

However, immutable variables can be 'shadowed':

```nu
let val = 42
do { let val = 101; $val }
# => 101
$val
# => 42
let val = $val + 1
$val
# => 43
```

### Mutable Variables

A mutable variable can change its value:

```nu
mut val = 42
$val += 27
$val
# => 69
```

Assignment operators:
- `=` - Assigns a new value
- `+=` - Adds to the variable
- `-=` - Subtracts from the variable
- `*=` - Multiplies the variable
- `/=` - Divides the variable
- `++=` - Appends a list or value

> **Note**
> Closures and nested `def`s cannot capture mutable variables from their environment.

### Constant Variables

A constant variable can be fully evaluated at parse-time:

```nu
const script_file = 'path/to/script.nu'
source $script_file
```

Useful with commands that need parse-time values: `source`, `use`, `plugin use`.

## Choosing between mutable and immutable variables

Try to use immutable variables for most use-cases.

Nushell's functional style with immutability enables:
- Better performance
- Streaming support
- Parallelism with `par-each`

### Loop counters with enumerate

```nu
ls | enumerate | each { |elt| $"Item #($elt.index) is size ($elt.item.size)" }
```

### Using reduce

```nu
[one, two, three, four, five, six] | reduce {|current_item, max|
  if ($current_item | str length) > ($max | str length) {
    $current_item
  } else {
    $max
  }
}
# => three
```

### Performance Considerations

Immutable variables with filter commands are often more performant:

```nu
# Using for loop (slow)
timeit {
  mut randoms = []
  for _ in 1..50_000 {
    $randoms = ($randoms | append (random int))
  }
}
# => 1min 4sec

# Using each (fast)
timeit {
  let randoms = (1..50_000 | each {random int})
}
# => 19ms
```

## Variable Names

Variable names cannot contain:
```
. [ ( { + - * ^ / = ! < > & |
```

Variables can start with `$`:
```nu
let $var = 42
# identical to `let var = 42`
```
