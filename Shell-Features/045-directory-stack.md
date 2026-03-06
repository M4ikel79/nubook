# Directory Stack

Nushell provides a Directory Stack feature via the Standard Library.

## Import

```nu
use std/dirs
```

Add to config for persistent use.

## Commands

| Command | Description |
|---------|-------------|
| `dirs` | List directories on stack |
| `dirs add <path>` | Add directory, becomes active |
| `dirs drop` | Remove current, previous becomes active |
| `dirs goto n` | Jump to index n |
| `dirs next` | Cycle to next directory |
| `dirs prev` | Cycle to previous |

## Usage

```nu
cd ~
dirs
# => ╭───┬────────┬──────────╮
# => │ # │ active │   path   │
# => ├───┼────────┼──────────┤
# => │ 0 │ true   │ /home/... │
# => ╰───┴────────┴──────────╯

cd /other/path
dirs
# cd only changes active dir

dirs add ~/projects
dirs
# => ╭───┬────────┬────────────╮
# => │ # │ active │    path   │
# => ├───┼────────┼────────────┤
# => │ 0 │ false  │ /other/.. │
# => │ 1 │ true   │ ~/projects │
# => ╰───┴────────┴────────────╘

dirs add ~/scripts
dirs add ~
dirs
# Add multiple directories

dirs next      # Cycle to next
dirs goto 0    # Jump to index
dirs drop      # Remove current
```

## Shell Aliases

For "shells within shells" concept:
```nu
use std/dirs shells-aliases *
```

Aliases:
- `shells` - list directories (like `dirs`)
- `enter` - add and enter directory (`dirs add`)
- `dexit` - exit current (`dirs drop`)
- `g` - goto index
- `n` - next
- `p` - prev
