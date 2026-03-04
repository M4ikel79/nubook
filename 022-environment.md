# Environment

You can see current environment variables:

```nu
$env | table -e
```

Environment variables can be any type. They are converted to strings when sent to external applications.

## Setting Environment Variables

### $env.VAR assignment

```nu
$env.FOO = 'BAR'
$env.Path = ($env.Path | prepend 'C:\path\to\add')
```

### load-env

Set multiple environment variables at once:

```nu
load-env { "BOB": "FOO", "JAY": "BAR" }
```

### One-shot environment variables

```nu
FOO=BAR $env.FOO
# => BAR
```

Or using with-env:

```nu
with-env { FOO: BAR } { $env.FOO }
# => BAR
```

## Reading Environment Variables

```nu
$env.FOO
# => BAR
```

Using optional operator for potentially unset variables:

```nu
$env.FOO? | default "BAR"
# => BAR
```

Check presence:

```nu
if "FOO" in $env { echo $env.FOO }
```

## Case Sensitivity

Nushell's `$env` is case-insensitive. `$env.PATH`, `$env.Path`, and `$env.path` all work the same.

## Scoping

Environment variables are scoped to the current block:

```nu
$env.FOO = "BAR"
do {
    $env.FOO = "BAZ"
    $env.FOO == "BAZ"
}
# => true
$env.FOO == "BAR"
# => true
```

## Changing the Directory

`cd` sets the `PWD` environment variable, so it follows the same scoping rules.

## Permanent Environment Variables

Set in config.nu:

```nu
# In config.nu
$env.FOO = 'BAR'
```

## Environment Variable Conversions

Convert between strings and other types for external commands:

```nu
$env.ENV_CONVERSIONS = {
    FOO: {
        from_string: { |s| $s | split row '-' }
        to_string: { |v| $v | str join '-' }
    }
}
```

Now `FOO=a-b-c` becomes `[a, b, c]` internally.

## Removing Environment Variables

```nu
$env.FOO = 'BAR'
hide-env FOO
```

Hiding is also scoped - prevents modifying parent environment from child scope.
