# Overlays

Overlays act as "layers" of definitions (custom commands, aliases, environment variables) that can be activated and deactivated on demand. They resemble virtual environments in languages like Python.

**Note:** To understand overlays, check Modules first as overlays build on top of modules.

## Basics

Nushell comes with one default overlay called `zero`. Inspect active overlays:
```nu
overlay list
```

### Creating an Overlay

First, create a module:
```nu
module spam {
    export def foo [] { "foo" }
    export alias bar = echo "bar"
    export-env { load-env { BAZ: "baz" } }
}
```

Then create the overlay:
```nu
overlay use spam

foo    # => foo
bar    # => bar
$env.BAZ  # => baz
```

This brings the module's definitions into scope and evaluates the `export-env` block.

## Removing an Overlay

```nu
overlay hide spam
```

Overlays are also scoped - they are removed at the end of the block:
```nu
do { overlay use spam; foo }  # overlay active only inside
```

Call `overlay hide` without an argument to remove the last active overlay.

## Overlays Are Recordable

Any new definition is recorded into the last active overlay:
```nu
overlay use spam
def eggs [] { "eggs" }  # added to spam overlay
overlay hide spam
eggs  # Error: can't run

overlay use spam
eggs  # works - eggs remembered
```

### Using a Scratchpad Overlay

Create an empty overlay for recording custom changes without affecting the original:
```nu
overlay use spam
overlay new scratchpad
def eggs [] { "eggs" }  # added to scratchpad, not spam
```

## Prefixed Overlays

Use `--prefix` to keep commands as subcommands:
```nu
module spam { export def foo [] { "foo" } }
overlay use --prefix spam
spam foo  # => foo
```

## Rename an Overlay

```nu
overlay use spam as eggs
eggs foo
overlay hide eggs
```

## Preserving Definitions

Use `--keep-custom` to keep custom definitions when hiding an overlay:
```nu
overlay use spam
def eggs [] { "eggs" }
overlay hide --keep-custom spam
eggs  # works - eggs moved to zero overlay
```

Use `--keep-env` to keep environment variables:
```nu
overlay use spam
overlay hide spam --keep-env [ FOO ]
$env.FOO  # preserved
```

## Ordering Overlays

Overlays are arranged as a stack. Last active takes precedence:
```nu
def foo [] { "foo-in-zero" }
overlay use spam
foo  # => foo (from spam)

overlay use zero
foo  # => foo-in-zero (zero now takes precedence)
```
