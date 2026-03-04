# Coming from PowerShell

**Key Difference**: PowerShell passes .NET objects through pipelines. Nushell passes **structured data** (tables, lists, values).

This means:
- No `.PropertyName` access
- Use `get`, `select`, or table operations instead

## Command Equivalents

| PowerShell | Nu | Task |
|------------|-----|------|
| `Get-ChildItem` | `ls` | List files |
| `Get-ChildItem <dir>` | `ls <dir>` | List directory |
| `Get-ChildItem *.md` | `ls *.md` | Pattern match |
| `ls -Force -Hidden` | `ls -la` | Hidden files |
| `ls \| where PSIsContainer` | `ls \| where type == dir` | Directories |
| `ls -Recurse -Filter *.rs` | `ls **/*.rs` | Recursive search |
| `Set-Location <dir>` | `cd <dir>` | Change directory |
| `Set-Location` | `cd` | Home directory |
| `Set-Location -` | `cd -` | Previous directory |
| `New-Item -ItemType Directory` | `mkdir` | Create directory |
| `New-Item test.txt` | `touch test.txt` | Create file |
| `cmd \| Out-File <path>` | `cmd \| save <path>` | Save output |
| `cmd \| Out-File -Append` | `cmd \| save --append` | Append output |
| `cmd \| Out-Null` | `cmd \| ignore` | Discard output |
| `cmd \| Tee-Object -File` | `cmd \| tee { save log.txt }` | Tee to file |
| `cmd \| Select -First 5` | `cmd \| first 5` | First N rows |
| `Get-Content <file>` | `open --raw <file>` | Read file |
| `Move-Item` | `mv` | Move file |
| `foreach ($i in 1..10)` | `for i in 1..10 { }` | Loop |
| `Copy-Item` | `cp` | Copy |
| `Copy-Item -Recurse` | `cp -r` | Recursive copy |
| `Remove-Item` | `rm` | Remove |
| `Remove-Item -Recurse` | `rm -r` | Recursive remove |
| `Get-Date` | `"<date>" \| into datetime` | Parse date |
| `$str -replace 'a','b'` | `str replace a b` | Replace |
| `Select-String` | `find` or `where =~` | Search |
| `Get-Help` | `help` | Help |
| `Get-Command` | `help commands` | List commands |
| `$env:Path` | `$env.PATH` | Environment |
| `$LASTEXITCODE` | `$env.LAST_EXIT_CODE` | Exit code |
| `Get-ChildItem Env:` | `$env` | List env vars |
| `$env:FOO` | `$env.FOO` | Get env var |
| `Remove-Item Env:FOO` | `hide-env FOO` | Unset env var |
| `Set-Alias s git status` | `alias s = git status` | Create alias |
| `which FOO` | `which FOO` | Find command |
| `.\script.ps1` | `nu script.nu` | Run script |
| `Get-Location` / `$PWD` | `pwd` | Current directory |
| `Read-Host` | `input` | User input |
| `Read-Host -AsSecureString` | `input -s` | Secret input |

## Pipeline Differences

PowerShell:
```powershell
Get-Process | Where-Object {$_.Name -eq "code"}
```

Nushell:
```nu
ps | where name == code
```
