# Testing your Nushell Code

Nushell provides assertion commands in the standard library for testing.

## Importing Assertions

```nu
use std/assert
```

## Base Assert Command

The foundation for every assertion - throws error if condition is not true:
```nu
assert (1 == 2)
# Error: Assertion failed.
```

With custom message:
```nu
let a = 0
assert ($a == 19) $"Expected 19, got: ($a)"
```

## Built-in Assert Commands

Nushell has many specialized assert commands with better error messages:

```nu
# Assert string contains
assert str contains "haystack" "needle"

# Assert equal
assert equal 1 1

# Assert not equal
assert not equal 1 2
```

See all available: `scope commands | where name =~ ^assert`

## Custom Assert Commands

Create custom assertions:
```nu
def "assert even" [number: int] {
    assert ($number mod 2 == 0) --error-label {
        text: $"($number) is not an even number",
        span: (metadata $number).span,
    }
}

assert even 13
# Error: 13 is not an even number
```

## Running Tests

### Nupm Package

If using Nupm package:
1. Create `tests/` directory next to `nupm.nuon`
2. Add `mod.nu` file to make it a module
3. Write test commands inside `tests/`
4. Run `nupm test`

Convention: Exported commands starting with "test " are run as tests:
- `export def "test something"` in `tests/mod.nu` - runs
- `def internal-cmd` - does NOT run
- `export def "test x"` in `tests/spam.nu` - runs if exported from mod.nu

### Standalone Tests

For non-Nupm projects, create test scripts:

```nu
# math.nu module
export def fib [n: int] {
    if $n == 0 { return 0 }
    if $n == 1 { return 1 }
    (fib ($n - 1)) + (fib ($n - 2))
}

# tests.nu
use math.nu fib
use std/assert

for t in [[input, expected]; [0 0] [1 1] [2 1] [3 2] [4 3]] {
    assert equal (fib $t.input) $t.expected
}
```

Run with: `nu tests.nu`

### Basic Test Framework

Discover tests dynamically:
```nu
use std/assert
source fib.nu

def main [] {
    let test_commands = (
        scope commands
            | where ($it.type == "custom") and ($it.name | str starts-with "test ")
            | get name
            | each { |test| [$"print 'Running: ($test)'", $test] }
            | flatten
            | str join "; "
    )
    nu --commands $"source fib.nu; ($test_commands)"
}

def "test fib" [] {
    assert equal (fib 0) 0
    assert equal (fib 1) 1
    assert equal (fib 10) 55
}

# ignore
def "test ignore-me" [] { }
```

This runs all commands starting with "test " except those marked with "ignore" in description.
