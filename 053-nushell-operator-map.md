# Nushell Operator Map

Quick reference for Nushell operators vs other languages.

## Comparison Operators

| Nushell | SQL | Python | .NET LINQ | PowerShell | Bash |
|---------|-----|--------|-----------|-----------|------|
| `==` | `=` | `==` | `==` | `-eq` | `==` |
| `!=` | `!=`, `<>` | `!=` | `!=` | `-ne`, `-isnot` | `!=` |
| `<` | `<` | `<` | `<` | `-lt` | `<` |
| `<=` | `<=` | `<=` | `<=` | `-le` | `<=` |
| `>` | `>` | `>` | `>` | `-gt` | `>` |
| `>=` | `>=` | `>=` | `>=` | `-ge` | `>=` |
| `=~` | like | re, in | Contains | `-like`, `-contains` | `=~` |
| `!~` | not like | - | Except | `-notlike`, `-notcontains` | `!~` |

## Math Operators

| Nushell | Python | PowerShell | Bash |
|---------|--------|-----------|------|
| `+` | `+` | `+` | `+` |
| `-` | `-` | `-` | `-` |
| `*` | `*` | `*` | `*` |
| `/` | `/` | `/` | `/` |
| `**` | `pow` | `Power` | `**` |

## Containment

| Nushell | SQL | Python | .NET | PowerShell |
|---------|-----|--------|------|------------|
| `in` | in | in | Contains | `-in` |
| `not-in` | not in | not in | Except | `-notin` |

## Boolean

| Nushell | Python | PowerShell | Bash |
|---------|--------|-----------|------|
| `and` | and | `-and`, `&&` | `-a`, `&&` |
| `or` | or | `-or`, `||` | `-o`, `||` |

## Regex/Pattern

| Nushell | Description |
|---------|-------------|
| `=~` | Regex match |
| `!~` | Not match |
| `=~ "pattern"` | Like SQL LIKE |
| `!~ "pattern"` | Not LIKE |
