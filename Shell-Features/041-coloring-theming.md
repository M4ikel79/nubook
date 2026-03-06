# Coloring and Theming in Nu

Customize colors in `config.nu` via `$env.config.color_config`.

## Table Borders

```nu
$env.config.table.mode = 'rounded'  # options: ascii_rounded, basic, compact, dots, double, heavy, light, markdown, none, psql, rounded, single, etc.
```

## Color Configuration

Colors defined in `$env.config.color_config`. Multiple formats:

- `red` - color name
- `red_bold` - color with attribute
- `"#ff0000"` - hex format (quotes required)
- `{ fg: "#ff0000", bg: "#0000ff", attr: b }` - full format
- `{|x| 'yellow' }` - closure (for table output only)

### Attributes
- `b` - bold
- `d` - dimmed
- `i` - italic
- `u` - underline
- `r` - reverse
- `s` - strikethrough
- `h` - hidden
- `l` - blink

### Color Codes
- `r`/`red` - red
- `g`/`green` - green
- `b`/`blue` - blue
- `u` - blue (bright)
- `c`/`cyan` - cyan
- `m`/`magenta` - magenta
- `y`/`yellow` - yellow
- `w`/`white` - white
- `ligr`/`light_gray` - light gray
- `dgr`/`dark_gray` - dark gray
- Prefix with `bg_` for background

### Closure Example
```nu
$env.config.color_config.filesize = {|x| if $x < 1mb { 'cyan' } else { 'blue' } }
$env.config.color_config.bool = {|x| if $x { 'green' } else { 'red' } }
```

## Primitive Value Colors

Configurable primitives:
- `string`, `int`, `float`, `bool`, `nothing`
- `filesize`, `duration`, `range`, `binary`
- `cell-path`, `datetime`, `list`, `record`
- `separator`, `header`, `empty`, `row_index`, `hints`

```nu
$env.config.color_config.string = white
$env.config.color_config.int = green
$env.config.color_config.bool = red
$env.config.color_config.filesize = cyan
```

## Shape Colors (Syntax Highlighting)

- `shape_bool`, `shape_int`, `shape_float`
- `shape_string`, `shape_variable`, `shape_operator`
- `shape_flag`, `shape_garbage`, `shape_external`

```nu
$env.config.color_config.shape_garbage = { fg: "#FFFFFF" bg: "#FF0000" attr: b }
$env.config.color_config.shape_string = green
```

## Prompt Configuration

```nu
$env.PROMPT_COMMAND = { $"(date now): (pwd | path basename)" }
$env.PROMPT_INDICATOR = "> "
$env.config.render_right_prompt_on_last_line = true
```

### Transient Prompt

```nu
$env.TRANSIENT_PROMPT_COMMAND = ""  # Minimal prompt for history
```

## LS_COLORS for `ls`

```nu
$env.LS_COLORS = "di=1;34:*.nu=3;33"  # Bold blue dirs, italic yellow .nu files
```

Format: `selector=attributes` (semicolon-separated)
- File types: `di`=directory, `fi`=file, `ln`=symlink
- Extensions: `*.nu`

Use `vivid` tool for themes:
```nu
$env.LS_COLORS = (vivid generate molokai)
```

## Theming Example (Base16)

```nu
let base16_theme = {
    separator: "#585858"
    header: "#a1b56c"
    shape_bool: "#7cafc2"
    shape_string: "#a1b56c"
    # ... etc
}
$env.config.color_config = $base16_theme
```

## Accessibility

Disable decorations:
```nu
$env.config.table.mode = "none"
$env.config.error_style = "plain"
```

## Light/Dark Themes

```nu
use std/config light-theme
$env.config.color_config = (light-theme)
# or
$env.config.color_config = (dark-theme)
```
