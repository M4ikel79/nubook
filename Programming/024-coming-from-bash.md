# Coming from Bash

This chapter covers the differences between Bash and Nushell and how to accomplish common tasks.

## Command Equivalents

| Bash | Nu | Task |
|------|-----|------|
| `ls` | `ls` | List files |
| `ls <dir>` | `ls <dir>` | List files in directory |
| `ls pattern*` | `ls pattern*` | List files matching pattern |
| `ls -la` | `ls -la` | List all files with details |
| `ls -d */` | `ls \| where type == dir` | List directories |
| `find . -name *.rs` | `ls **/*.rs` | Find files recursively |
| `cd <directory>` | `cd <directory>` | Change directory |
| `cd` | `cd` | Change to home |
| `cd -` | `cd -` | Change to previous directory |
| `mkdir <path>` | `mkdir <path>` | Create directory |
| `mkdir -p <path>` | `mkdir <path>` | Create directory with parents |
| `> <path>` | `out> <path>` or `o> <path>` | Save to file |
| `>> <path>` | `out>> <path>` or `o>> <path>` | Append to file |
| `> /dev/null` | `\| ignore` | Discard output |
| `command 2>&1 \| less` | `command out+err>\| less` | Pipe stdout and stderr |
| `command \| tee log.txt` | `command \| tee { save log.txt }` | Tee output to file |
| `cat <path>` | `open --raw <path>` | Display file contents |
| `mv <source> <dest>` | `mv <source> <dest>` | Move file |
| `for f in *.md; do echo $f; done` | `ls *.md \| each { $in.name }` | Iterate over files |
| `for i in $(seq 1 10); do echo $i; done` | `for i in 1..10 { print $i }` | Loop with numbers |
| `cp <source> <dest>` | `cp <source> <dest>` | Copy file |
| `rm <path>` | `rm <path>` | Remove file |
| `rm -rf <path>` | `rm -r <path>` | Remove recursively |
| `sed` | `str replace` | Find and replace |
| `grep <pattern>` | `where $it =~ <pattern>` or `find <pattern>` | Filter strings |
| `man <command>` | `help <command>` | Get help |
| `command1 && command2` | `command1; command2` | Run sequentially |
| `stat $(which git)` | `stat ...(which git).path` | Command output as argument |
| `echo $PATH` | `$env.PATH` | See current path |
| `echo $?` | `$env.LAST_EXIT_CODE` | See exit status |
| `export PATH=$PATH:/new` | `$env.PATH = ($env.PATH \| append /new)` | Add to PATH |
| `export` | `$env` | List environment |
| `alias s="git status"` | `alias s = git status` | Create alias |
| `source script.sh` | `source script.nu` | Source script |
| `bash -c <commands>` | `nu -c <commands>` | Run commands |
| `bash <script>` | `nu <script>` | Run script |

## Environment Variables

```nu
# Set temporarily
$env.FOO = "BAR"

# Use
$env.FOO

# Fallback for unset
$env.FOO? | default "ABC"

# Unset
hide-env FOO
```

## History

| Bash | Nu | Task |
|------|-----|------|
| `!!` | `!!` | Last command |
| `!$` | `!$` | Last argument |
| `Ctrl+R` | `Ctrl+R` | Reverse search |
| `Ctrl+X Ctrl+E` | `Ctrl+O` | Edit in editor |

## Notes

- History substitution in Nushell inserts into command-line first, allowing review before execution
- Use `^` to run external commands: `^grep`
- Nushell uses pipelines with structured data, not just text
- Variables use `$` prefix: `let $x = 5`
- Environment variables use `$env.`
