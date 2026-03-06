# Externs

Externs allow defining full signatures for external commands, enabling:
- Parse-time type checking
- Completions
- Syntax highlighting

## Basic Example

```nu
module "ssh extern" {
  def complete_none [] { [] }
  
  def complete_ssh_identity [] {
    ls ~/.ssh/id_*
    | where { ($in.name | path parse | get extension) != "pub" }
    | get name
  }

  export extern ssh [
    destination?: string@complete_none   # Destination Host
    -p: int                              # Destination Port
    -i: string@complete_ssh_identity     # Identity File
  ]
}
use "ssh extern" ssh
```

This provides:
- `-p` and `-i` flags with completions
- Parse-time type checking (port must be int)
- Syntax highlighting based on argument shapes
- Private key files as completions for `-i`

**Note:** Comment for documentation needs space before `#`.

## Format Specifiers

- Optional parameter: `?` after type
- Rest parameters: `...` before parameter name
```nu
export extern "git add" [
  ...pathspecs: path
]
```

## Limitations

1. Can't enforce flag/argument ordering
2. Can't require `=` for flag values
3. `^command` (caret sigil) externals not recognized
4. Can't represent `-long` arguments with single hyphen
