# Coming from CMD.EXE

## Command Equivalents

| CMD.EXE | Nu | Task |
|---------|-----|------|
| `ASSOC` | | File extension associations |
| `CALL <file>` | `source <file>` / `use <file>` | Run script |
| `CD` | `$env.PWD` | Current directory |
| `CD <dir>` | `cd <dir>` | Change directory |
| `CLS` | `clear` | Clear screen |
| `COLOR` | `ansi` | Set colors |
| `COPY <src> <dst>` | `cp <src> <dst>` | Copy files |
| `DATE /T` | `date now` | Current date |
| `DEL <file>` | `rm <file>` | Delete files |
| `DIR` | `ls` | List files |
| `ECHO <msg>` | `print <msg>` | Print output |
| `ECHO ON` | | Echo commands |
| `ENDLOCAL` | `export-env` | Export env changes |
| `EXIT` | `exit` | Exit |
| `FOR %i IN (set) DO cmd` | `for $i in <set> { cmd }` | Loop |
| `GOTO` | | Jump to label |
| `IF ERRORLEVEL n cmd` | `if $env.LAST_EXIT_CODE >= n { cmd }` | Error check |
| `IF EXIST file cmd` | `if (file | path exists) { cmd }` | File exists |
| `MD` or `MKDIR` | `mkdir` | Make directory |
| `MKLINK` | | Create symlink |
| `MOVE` | `mv` | Move files |
| `PATH` | `$env.Path` | Path variable |
| `PAUSE` | `input "Press key..."` | Pause |
| `PROMPT` | `$env.PROMPT_COMMAND` | Customize prompt |
| `PUSHD`/`POPD` | `enter`/`dexit` | Temp directory |
| `REM` | `#` | Comments |
| `REN` | `mv` | Rename |
| `RD` | `rm` | Remove directory |
| `SET var=value` | `$env.var = value` | Set env var |
| `SETLOCAL` | (default) | Localize changes |
| `START` | `start` | Open application |
| `TIME /T` | `date now | format date "%H:%M:%S"` | Current time |
| `TYPE <file>` | `open --raw <file>` | Display file |
| `VER` | | Version |

## Forwarded CMD.EXE Commands

Nu runs some CMD.EXE internal commands via `cmd.exe`:
- `ASSOC`, `CLS`, `ECHO`, `FTYPE`, `MKLINK`, `PAUSE`, `START`, `VER`, `VOL`

Use `^command` to force CMD.EXE version: `^ver`
