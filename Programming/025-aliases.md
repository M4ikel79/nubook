# Aliases

Aliases provide a way to create shorthand names for longer commands.

## Creating Aliases

```nu
alias ll = ls -l
```

Now you can use:
```nu
ll -a  # Equivalent to ls -l -a
```

## List All Loaded Aliases

```nu
scope aliases
help aliases
```

## Persisting Aliases

Add to config.nu:
```nu
config nu  # Opens editor
# Add: alias ll = ls -l
```

## Piping in Aliases

Aliases can't use pipes directly. Use a custom command instead:

```nu
def uuidgen [] { ^uuidgen | tr A-F a-f }
```

## Replacing Existing Commands

Caution: Backup the original command first!

```nu
alias core-ls = ls  # Backup ls
alias ls = ls -la    # Replace with new version
```

To avoid recursion, use aliases (not def) for replacement:

```nu
alias ls-builtin = ls

def ls [
    --all (-a)
    --long (-l)
    ...pattern: glob
]: [nothing -> table] {
    ls-builtin --all=$all --long=$long ...$pattern | sort-by type name
}
```
