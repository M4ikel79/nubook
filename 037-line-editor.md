# Reedline, Nu's Line Editor

Reedline is Nushell's cross-platform line editor. It handles command history, completions, hints, and screen painting.

## Multi-line Editing

Reedline allows commandlines to extend across multiple lines:

1. **Press Enter with open brackets** - auto-inserts newline for `{}`, `()`, `[]`
   ```nu
   def my-command [] {
   ```

2. **Trailing pipe symbol** - press Enter after `|`
   ```nu
   ls |
   where name =~ 'regex'
   ```

3. **Alt+Enter or Shift+Enter** - manually insert newline

4. **Ctrl+O** - opens current line in your editor

## Editing Modes

Set mode in config:
```nu
$env.config.edit_mode = 'vi'  # or 'emacs' (default)
```

## Keybindings

### Emacs/Vi-insert Mode
- `Ctrl+C` - Cancel line
- `Ctrl+L` - Clear screen
- `Ctrl+R` - Search history
- `Ctrl+P/N` - Move up/down history
- `Ctrl+←/→` - Move by word
- `↑/↓` - Navigate history
- `Home/End` - Line start/end

### Vi-normal Mode
- `Esc` - Switch to normal mode
- `h/j/k/l` - Move left/down/up/right
- `w/e/b` - Word navigation
- `d` - Delete
- `p` - Paste
- `i` - Insert mode
- `u` - Undo

## Command History

```nu
$env.config.history.max_size = 1000
```

## Keybinding Configuration

Add custom keybindings:
```nu
$env.config.keybindings ++= [{
    name: completion_menu
    modifier: control
    keycode: char_t
    mode: emacs
    event: { send: menu name: completion_menu }
}]
```

Each keybinding needs: name, modifier, keycode, mode, event.

### Event Types
- `send` - Send an event like Enter, menu commands
- `edit` - Edit commands like insert, delete
- `until` - Try events until one succeeds

### Remove Default Keybinding
```nu
$env.config.keybindings ++= [{
    modifier: control
    keycode: char_l
    mode: [emacs, vi_normal, vi_insert]
    event: null
}]
```

## Menus

### Help Menu (F1)
Search commands by keywords, shows name and description.

### Completion Menu (Tab)
Context-sensitive suggestions for commands, paths, flags.

### History Menu (Ctrl+R)
Search command history in reverse chronological order.

Quick select with `!number` + Enter.

### Custom Menus

```nu
$env.config.menus ++= [{
    name: my_menu
    only_buffer_difference: true
    marker: "# "
    type: { layout: list, page_size: 10 }
    style: { text: green, selected_text: green_reverse }
    source: { |buffer, position|
        scope variables
        | where name =~ $buffer
        | each { |row| {value: $row.name, description: $row.type} }
    }
}]
```

Menu source returns records with:
- `value`: Inserted in buffer
- `description`: Displayed with value
- `span`: Replacement range
- `extra`: Extra strings (description menu only)
