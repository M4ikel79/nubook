# Running System (External) Commands

Nu provides cross-platform internal commands, but you can also run external system commands.

## Calling External Commands

Use the caret (^) sigil to run external commands:

```nu
ls          # Internal Nu command
^ls         # External /bin/ls
```

## External vs Internal

| Command | Description |
|--------|-------------|
| `ls` | Nushell's built-in ls |
| `^ls` | System's /usr/bin/ls |
| `date` | Nushell's date |
| `^date` | System's date command |

## Windows Notes

On Windows, some CMD.EXE commands are forwarded to cmd instead of running externally. See "Coming from CMD.EXE" for details.

## Passing Arguments

```nu
^grep -r "pattern" .
^cargo build --release
```

## Capturing Output

```nu
let content = (^cat file.txt)
```

## Environment

External commands inherit environment from Nushell. Use `with-env` for temporary changes:

```nu
with-env { VAR: "value" } { ^command }
```
