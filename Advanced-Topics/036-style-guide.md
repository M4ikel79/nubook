# Best Practices

Guidelines and best practices for writing Nushell code.

## Formatting

### Defaults

Assume no spaces by default, except where rules define otherwise.

### Basic Rules

- Put one space before and after pipe `|`, commands, options, and arguments
- Never put several consecutive spaces unless part of a string
- Omit commas between list items

Correct:
```nu
'Hello!' | ansi gradient --fgstart '0x40c9ff' --fgend '0xe81cff'
```

Incorrect:
```nu
'Hello!' |  ansi gradient  # too many spaces
```

### One-line Format

Default format for:
- Pipelines less than 80 chars
- Lists/records under 80 chars without nesting

Rules:
- One space after comma for block/closure parameters
- One space after pipe `|` in parameter lists
- One space after opening brace `{` if no parameters
- One space before closing brace `}`
- One space after `:` in records
- One space after comma in lists/records

Correct:
```nu
[[status]; [UP] [UP]] | all {|el| $el.status == UP }
[1 2 3 4] | reduce {|elt, acc| $elt + $acc }
{x: 1, y: 2}
[]
(1 + 2) * 3
```

### Multi-line Format

Use for:
- Scripts
- Lists/records over 80 chars or with nesting
- Pipelines over 80 chars

Rules:
- Omit trailing spaces
- Each pipeline on separate line in blocks
- Each key-value on separate line in records
- Each item on separate line in lists

```nu
[[status]; [UP] [UP]] | all {|el|
    $el.status == UP
}

[
  {name: "Teresa", age: 24}
]
```

## Naming Convention

### Abbreviations

Use full words over abbreviations unless well-known:
```nu
query-user --id 123
$user.name | str downcase
```

### Case

**Commands**: kebab-case
```nu
fetch-user --id 123
```

**Sub-commands**: kebab-case
```nu
date now
date list-timezone
def "login basic-auth" [username: string password: string] { }
```

**Flags**: kebab-case
```nu
def greet [name: string, --all-caps] { }
```

**Variables/Parameters**: snake_case
```nu
let user_id = 123
def fetch-user [user_id: int] { }
```

**Environment Variables**: SCREAMING_SNAKE_CASE
```nu
$env.APP_VERSION = "1.0.0"
```

## Command Design

- Keep positional parameters ≤ 2; use options for more
- Use positional params unless optional parameters need different kinds
- Provide both long and short options
- Provide documentation for all exported entities

## Documentation

Recommend providing documentation for:
- Custom commands
- Parameters and options
- Module-level docs
