# Standard Library (Preview)

Nu ships with a standard library of useful commands written in native Nu. It's loaded but not auto-imported on startup.

## Available Modules

- Assertions (`std/assert`)
- Help system with completions (`std/help`)
- JSON formats (`std/formats`)
- XML access (`std/xml`)
- Logging (`std/log`)
- Directory stack (`std/dirs`)
- Input/Output (`std/input`)
- Iterators (`std/iters`)
- Math constants (`std/math`)
- Date/time (`std/dt`)

List all commands:
```nu
nu -c "use std; scope commands | where name =~ '^std ' | table"
```

## Importing

### Submodule form (recommended - faster)
```nu
use std/assert
use std/log
use std/dirs
use std/help
```

### With glob for definitions
```nu
use std/formats *
use std/dt *
use std/math *
use std/xml *
```

### Entire library (slower)
```nu
use std *  # Avoid in scripts
```

**Tip:** Use slash form (`std/log`) not space form (`use std log`) for better performance.

## Disable Standard Library

```nu
nu --no-std-lib
```

This speeds up startup significantly.

## std/log Usage

```nu
use std/log
log info "Message"
log warning "Warning"
log error "Error"
```

Set log level:
```nu
NU_LOG_LEVEL=DEBUG nu script.nu
```

## Optimal Startup

Avoid loading entire stdlib. Use specific imports:
```nu
# Bad (loads all)
use std *

# Good (loads only what needed)
use std/log
use std/dirs
```

Check config for inefficient imports:
```nu
view files | enumerate | flatten | where filename !~ '^std' | where {|f| (view span $f.start $f.end) =~ 'use\\W+std[^\\/]' }
```

## std-rfc

Staging area for potential stdlib additions. Found in nushell repository. Submit PRs to contribute.
