# explore

Explore is a table pager, just like `less` but for table structured data.

## Signature

`> explore --head --index --reverse --peek`

### Parameters

- `--head {bool}`: Show or hide column headers (default true)
- `--index, -i`: Show row indexes when viewing a list
- `--tail, -t`: Start with the viewport scrolled to the bottom
- `--peek, -p`: When quitting, output the value of the cell the cursor was on

## Get Started

```nu
ls | explore -i
```

## Navigation

- Arrow keys: `<Left>`, `<Right>`, `<Up>`, `<Down>`
- Vim keybindings: `h`, `j`, `k`, `l`, `Ctrl-f`, `Ctrl-b`
- Emacs keybindings: `Ctrl-v`, `Alt-v`, `Ctrl-p`, `Ctrl-n`

## Inspecting Values

Press `i` or `<Enter>` to enter cursor mode, then use arrow keys to choose a cell and see its underlying structure.

## Commands

Type `:help` after running explore to see the comprehensive list of commands.

## Config

Configure styles and colors via config. See example configuration in `default-config.nu`.

## Examples

### Peeking a Value

```nu
$nu | explore --peek
```

### :try Command

There's an interactive environment to navigate through data using nu. Combine with `--peek` to keep the chosen value.
