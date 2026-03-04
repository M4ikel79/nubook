# Configuration

## Quickstart

1. Set your preferred editor:
```nu
$env.config.buffer_editor = "code"  # or "nano", "vim", etc.
```

2. Edit config using:
```nu
config nu
```

3. Add startup commands to this file

4. Save and start a new Nushell session

## Configuration Files

Nushell uses multiple configuration files loaded in order:
1. `env.nu` - Environment variables (legacy)
2. `config.nu` - Nushell settings and startup tasks
3. `*.nu` in vendor-autoload-dirs - Vendor/package manager files
4. `*.nu` in user-autoload-dirs - User startup files
5. `login.nu` - Login shell specific config

Default location: `$nu.default-config-dir` (e.g., `~/.config/nushell`)

## Common Configuration Tasks

### Set Environment Variables

```nu
$env.PATH ++= ["~/.local/bin"]
```

Using Standard Library:
```nu
use std/util "path add"
path add "~/.local/bin"
```

### Prompt Configuration

Environment variables for prompts:
- `$env.PROMPT_COMMAND` - Main prompt (string or closure)
- `$env.PROMPT_COMMAND_RIGHT` - Right-side prompt
- `$env.PROMPT_INDICATOR` - Emacs mode indicator
- `$env.PROMPT_INDICATOR_VI_NORMAL` - Vi-normal mode indicator
- `$env.PROMPT_INDICATOR_VI_INSERT` - Vi-insert mode indicator
- `$env.PROMPT_MULTILINE_INDICATOR` - Multi-line indicator

Disable right prompt:
```nu
$env.PROMPT_COMMAND_RIGHT = ""
```

### Transient Prompts

For after command execution:
- `$env.TRANSIENT_PROMPT_COMMAND`
- `$env.TRANSIENT_PROMPT_COMMAND_RIGHT`
- etc.

### ENV_CONVERSIONS

Convert string environment variables to Nushell types:
```nu
$env.ENV_CONVERSIONS = $env.ENV_CONVERSIONS | merge {
    "XDG_DATA_DIRS": {
        from_string: {|s| $s | split row (char esep) }
        to_string: {|v| $v | str join (char esep) }
    }
}
```

## Nushell Settings ($env.config)

Change settings:
```nu
$env.config.show_banner = false
$env.config.buffer_editor = "code"
$env.config.history = {
    file_format: sqlite
    max_size: 1_000_000
    sync_on_enter: true
}
```

View all settings:
```nu
$env.config | table -e | less -R
# or
config nu --doc | nu-highlight | less -R
```

## Startup Variables

- `$env.XDG_CONFIG_HOME` - Config directory
- `$env.XDG_DATA_HOME` - Data directory
- `$env.XDG_DATA_DIRS` - Vendor autoload directories

## Constants

Use constants with parse-time keywords like `source` and `use`:
```nu
source ($nu.default-config-dir | path join "myfile.nu")
```

### NU_LIB_DIRS
```nu
const NU_LIB_DIRS = ['~/myscripts']
source myscript.nu
```

### NU_PLUGIN_DIRS
```nu
const NU_PLUGIN_DIRS = [
  ($nu.current-exe | path dirname)
  ($nu.data-dir | path join 'plugins' | path join (version).version)
]
```

## Remove Welcome Message

```nu
$env.config.show_banner = false
```

## macOS: Keeping /usr/bin/open

```nu
alias nu-open = open
alias open = ^open
```

## Detailed Startup Process

Launch stages:
1. Inherit environment from parent process
2. Get configuration directory
3. Initialize `$env.NU_LIB_DIRS`
4. Load Standard Library
5. Load env.nu
6. Load config.nu
7. Load login.nu (if login shell)
8. Load autoload directories

## Flag Behavior

- `nu` - Normal shell with all config
- `nu --login` / `nu -l` - Login shell
- `nu -c "command"` - Command string mode
- `nu -n` - No config files
- `nu --no-std-lib` - No standard library
- `nu --config <file>` - Custom config
