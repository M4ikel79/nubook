# Modules

Modules are containers that hold various definitions:
- Custom commands
- Aliases
- Constants
- Externs
- Environment variables
- Other modules (submodules)

## Using Modules

Import a module:
```nu
use <module>
```

## Creating Modules

Create a module file:

```nu
# mymodule.nu
export def hello [] { "Hello!" }
export const my_const = 42
export alias my_alias = ls
```

Use the module:
```nu
use mymodule.nu
mymodule hello
```

## Inline Modules

```nu
module greetings {
    export def hello [name: string] {
        $"Hello, ($name)!"
    }
    export def hi [where: string] {
        $"Hi, ($where)!"
    }
}

use greetings hello
hello "World"
```

## Module with Environment

```nu
# env.nu
export-env {
    $env.MYNAME = "Arthur"
}

export def hello [] {
    $"Hello, ($env.MYNAME)"
}
```

Use with environment:
```nu
use env.nu
$env.MYNAME  # "Arthur"
greetings hello
```

## Main Command

```nu
# main.nu
export def hello [name: string] { $"Hello, ($name)!" }
export def main [] { "Greetings!" }

use main.nu
main  # "Greetings!"
main hello "World"  # "Hello, World!"
```
