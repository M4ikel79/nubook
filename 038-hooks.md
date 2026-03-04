# Hooks

Hooks run code snippets at predefined situations in REPL mode. They don't work in script mode.

## Hook Types

- `pre_prompt`: Before prompt is drawn
- `pre_execution`: Before line executes
- `env_change`: When environment variable changes
- `display_output`: Block to pass output through
- `command_not_found`: When command not found

## Execution Cycle

1. Check for `pre_prompt` hooks
2. Check for `env_change` hooks
3. Display prompt, wait for input
4. Check for `pre_execution` hooks
5. Parse and evaluate input
6. Run `command_not_found` if applicable
7. Run `display_output` if defined
8. Repeat

## Basic Hooks

```nu
$env.config.hooks = {
    pre_prompt: [{ print "pre prompt hook" }]
    pre_execution: [{ print "pre exec hook" }]
    env_change: {
        PWD: [{|before, after| print $"Changed: ($before) -> ($after)" }]
    }
}
```

## Lists of Hooks

```nu
$env.config.hooks = {
    pre_prompt: [
        { print "first hook" }
        { print "second hook" }
    ]
}
```

## Conditional Hooks

Using record format with `condition` and `code`:

```nu
$env.config.hooks = {
    env_change: {
        PWD: [{
            condition: {|before, after| $after == /special/dir }
            code: {|before, after| load-env { MYVAR: "value" } }
        }]
    }
}
```

## Hooks as Strings

Define code as string for more capabilities:

```nu
$env.config.hooks = {
    pre_prompt: '$env.MY_VAR = "set"'
}
```

This allows defining commands/aliases conditionally:

```nu
$env.config.hooks = {
    env_change: {
        PWD: [{
            condition: {|_, after| $after == /project/dir }
            code: 'def mycmd [] { print "hello" }'
        }]
    }
}
```

## Examples

### Auto-activate environment
```nu
$env.config.hooks.env_change.PWD = [{
    condition: {|_, after|
        ($after | path join .env.nu | path exists)
    }
    code: 'overlay use .env.nu'
}]
```

### Filter/diversion output
```nu
$env.config.hooks.display_output = {
    to html --partial --no-color | save /tmp/nu-output.html
}
```

### Custom display based on terminal
```nu
$env.config.hooks.display_output = {
    if (term size).columns >= 100 { table -ed 1 } else { table }
}
```

### Arch Linux command_not_found
```nu
$env.config.hooks.command_not_found = {|cmd_name|
    try {
        let pkgs = (pkgfile --binaries --verbose $cmd_name)
        if ($pkgs | is-empty) { return null }
        $"($cmd_name) may be found in: ($pkgs)"
    }
}
```

### NixOS command_not_found
```nu
$env.config.hooks.command_not_found = {|command_name|
    print (command-not-found $command_name | str trim)
}
```
