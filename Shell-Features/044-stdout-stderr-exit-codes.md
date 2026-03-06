# Stdout, Stderr, and Exit Codes

## Stdout

External commands send data to stdout. In a pipeline:
```nu
external | str join
```
Redirects stdout into the pipeline.

Without pipeline, prints directly to screen.

## Stderr

Error messages go to stderr by default (prints to screen). Redirect stderr:

- `e>|` - pass stderr to next command
- `e> file` - redirect stderr to file
- `do -i { cmd } | complete` - capture stderr

## Exit Code

Last exit code stored in `$env.LAST_EXIT_CODE`:
```nu
do { external }
$env.LAST_EXIT_CODE
```

Or use `complete` command:
```nu
cat unknown.txt | complete
# => {stdout: "", stderr: "No such file", exit_code: 1}
```

## print vs echo

- `echo` - for pipes, returns arguments
- `print` - prints to stdout, returns nothing

## Logging

```nu
use std/log
log debug "Debug message"
log info "Info message"
log warning "Warning message"
log error "Error message"
log critical "Critical message"
```

Set log level:
```nu
NU_LOG_LEVEL=DEBUG nu script.nu
```

## File Redirections

```nu
cat file.txt o> out.log      # stdout to file
cat file.txt e> err.log      # stderr to file
cat file.txt o+e> both.log   # both to same file
```

Short forms: `o>` = `out>`, `e>` = `err>`

## Pipe Redirections

```nu
cat unknown.txt e>| str upcase   # stderr to next command
cat file o+e>| str upcase         # combined stdout+stderr
```

## Raw Streams

External stdout/stderr are byte streams. Nushell converts to UTF-8 text.

Use `decode` for custom encoding:
```nu
0x[8a 4c] | decode shift-jis
# => 貝
```
