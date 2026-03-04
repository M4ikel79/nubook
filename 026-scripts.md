# Scripts

Nushell scripts are `.nu` files containing Nushell code.

## Running Scripts

```nu
nu myscript.nu       # Run in new instance
source myscript.nu     # Run in current instance
```

## Example Script

```nu
# myscript.nu
def greet [name] {
  ["hello" $name]
}

greet "world"
```

## How Scripts are Processed

1. Definitions run first
2. Then the main script runs from top to bottom

Definitions can appear anywhere in the script - they don't need to come before usage.

## Script Lines

```nu
a
b; c | d
```

- First runs `a`, views results
- Then runs `b; c | d` (pipeline)

## Parameterizing Scripts

Use a `main` command:

```nu
# myscript.nu
def main [x: int] {
  $x + 10
}
```

```nu
nu myscript.nu 100
# => 110
```

## Argument Type Interpretation

Arguments are `any` by default:

```nu
# implicit
def main [x] { $"($x | describe) ($x)" }
nu implicit.nu +1  # => "int +1"

# explicit
def main [x: string] { $"($x | describe) ($x)" }
nu explicit.nu +1  # => "string +1"
```

## Subcommands

```nu
# myscript.nu
def "main run" [] { print "running" }
def "main build" [] { print "building" }
def main [] { print "hello!" }
```

```nu
nu myscript.nu      # hello!
nu myscript.nu build # building
nu myscript.nu run   # running
```

Must define `main` for subcommands to work.

## Shebangs

```nu
#!/usr/bin/env nu
"Hello World!"
```

```nu
./myscript  # Run directly
```

For stdin:
```nu
#!/usr/bin/env -S nu --stdin
echo "Hello" | ./script.nu
```
