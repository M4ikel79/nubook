# Control Flow

Nushell provides several commands that help determine how different groups of code are executed.

## Choice (Conditionals)

### if

`if` evaluates branching blocks of code based on conditions:

```nu
if $x > 0 { 'positive' }
if $x > 0 { 'positive' } else { 'non-positive' }
if $x > 0 { 'positive' } else if $x == 0 { 'zero' } else { 'negative' }
```

> **Tips**
> The conditional commands are expressions, so they return values.

### match

`match` executes one of several conditional branches based on a value:

```nu
match 3 {
    1 => 'one',
    2 => { 't' + 'w' + 'o' },
    3 => 'three',
    _ => 'other'
}
# => three
```

Catch all branch:
```nu
match 7 { 1 => 'one', 2 => 'two', _ => 'other number' }
```

Pattern matching:
```nu
let foo = { name: 'bar', count: 7 }
match $foo {
    { name: 'bar', count: $it } => ($it + 3),
    { name: _, count: $it } => ($it + 7),
    _ => 1
}
# => 10
```

Guards:
```nu
match $foo {
    { name: 'bar', count: $it } if $it < 5 => ($it + 3),
    { name: 'bar', count: $it } if $it >= 5 => ($it + 7),
    _ => 1
}
```

## Loops

### for

```nu
for x in [1 2 3] { $x * $x | print }
```

### while

```nu
mut x = 0; while $x < 10 { $x = $x + 1 }; $x
# => 10
```

### loop

```nu
mut x = 0; loop { if $x > 10 { break }; $x = $x + 1 }; $x
# => 11
```

### break

```nu
for x in 1..10 { if $x > 3 { break }; print $x }
# => 1, 2, 3
```

### continue

```nu
mut x = -1; while $x <= 6 { $x = $x + 1; if $x mod 3 == 0 { continue }; print $x }
# => 1, 2, 4, 5, 7
```

## Loop Advantages vs Disadvantages

**Disadvantages of loops:**
- Statements don't work in pipelines
- Can't modify mutable variables from closures

**Advantages of loops:**
- Can use mutable variables
- Better for continuous scripts like prompts

## Errors

### error make

```nu
error make { msg: 'Some error info' }
```

### try

```nu
try { error make { msg: 'Some error info' }}; print 'Resuming'
# => Resuming

try { 1 / 0 } catch { 'An error happened!' }
# => An error happened!

try { ^nonexisting }; print 'a'
# => a
```

## Other

### return

```nu
def 'positive-check' [it] {
    if $it > 0 { return 'positive' }
    'non-positive'
}
```
