# Nu Map from Other Shells and DSLs

Quick reference mapping Nushell commands to other languages.

## Core Commands

| Nushell | SQL | .NET LINQ | PowerShell | Bash |
|---------|-----|-----------|------------|------|
| `alias` | alias | alias | alias | alias |
| `append` | Append | -Append | `-Append` | - |
| `cd` | - | Set-Location | Set-Location, cd | cd |
| `clear` | - | Clear-Host | Clear-Host | clear |
| `cp` | - | Copy-Item | Copy-Item, cp | cp |
| `ls` | - | Get-ChildItem | dir, ls | ls |
| `mkdir` | - | New-Item | mkdir, md | mkdir |
| `mv` | - | Move-Item | mv, move | mv |
| `rm` | - | Remove-Item | del, rm | rm |
| `cp -r` | - | Copy-Item -Recurse | cp -r | cp -r |
| `rm -r` | - | Remove-Item -Recurse | rm -r | rm -r |

## Data Processing

| Nushell | SQL | .NET LINQ | PowerShell | Bash |
|---------|-----|-----------|------------|------|
| `math avg` | avg | Average | Measure-Object | bc |
| `each` | - | ForEach-Object | foreach, for | for |
| `group-by` | group by | GroupBy | Group-Object | group |
| `select` | select | Select | Select-Object | - |
| `where` | where | Where | Where-Object | grep |
| `sort-by` | order by | OrderBy | Sort-Object | sort |
| `first` | top | First | Select-Object -First | head |
| `last` | Last | Last | Select-Object -Last | tail |
| `skip` | offset | Skip | Select-Object -Skip | tail -n+ |
| `take` | top | Take | Select-Object -First | head |
| `get` | - | - | (cmd).column | - |
| `reduce` | Aggregate | Aggregate | - | - |
| `uniq` | distinct | Distinct | Get-Unique | uniq |
| `reverse` | - | Reverse | - | tac |
| `shuffle` | - | Random | Sort-Object {Get-Random} | shuf |

## String/Data

| Nushell | SQL | .NET LINQ | PowerShell | Bash |
|---------|-----|-----------|------------|------|
| `str stats` | count | Count | Measure-Object | wc |
| `str length` | len | - | - | wc -c |
| `lines` | - | - | - | wc -l |
| `str join` | concat_ws | Join | Join-String | - |
| `str trim` | - | Trim | Trim | tr |
| `format` | - | String.Format | - | printf |
| `from` / `to` | - | ConvertFrom/To-Json | Import/Export | - |

## System

| Nushell | PowerShell | Bash |
|---------|------------|------|
| `date` | Get-Date | date |
| `ps` | Get-Process | ps |
| `pwd` | Get-Location | pwd |
| `kill` | Stop-Process | kill |
| `http` | Invoke-WebRequest | wget, curl |
| `sys mem` | Get-ComputerInfo | free |
| `uname` | Get-ComputerInfo | uname |
| `version` | $PSVersionTable | - |
| `history` | Get-History | history |
| `help` | Get-Help | man |
| `which` | Get-Command | which |

## Environment

| Nushell | PowerShell | Bash |
|---------|------------|------|
| `$env.FOO = "bar"` | $env:FOO = "bar" | export FOO=bar |
| `config` | $Profile | ~/.bashrc |
| `open` | Get-Content | cat |
| `save` | Out-File | > file |
| `touch` | Set-Content | touch |

## Control Flow

| Nushell | PowerShell | Bash |
|---------|------------|------|
| `if` | if | if |
| `match` | switch | case/esac |
| `for` | foreach | for |
| `while` | while | while |
| `do { }` | { } | { } |

## Ranges

| Nushell | Description |
|---------|-------------|
| `1..10` | Range 1 to 10 |
| `'a'..'f'` | Character range |
