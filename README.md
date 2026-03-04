# Welcome to Nushell Book (nubook)

A comprehensive collection of markdown files converting the official Nushell book (https://www.nushell.sh/book/) into learnable, offline-friendly chapters.

## About This Book

Nushell (or Nu) is a modern shell that combines the power of traditional shells with the capabilities of a programming language. This book covers everything from installation to advanced topics like plugins, parallelism, and dataframes.

---

## Table of Contents

### Getting Started

| # | File | Description |
|---|------|-------------|
| 001 | [introduction.md](001-introduction.md) | What is Nushell, philosophy, and key features |
| 002 | [installation.md](002-installation.md) | How to install Nushell on various platforms |
| 003 | [default-shell.md](003-default-shell.md) | Setting Nushell as your default shell |
| 004 | [getting-started.md](004-getting-started.md) | Quick start guide |
| 005 | [quick-tour.md](005-quick-tour.md) | Overview of Nushell's capabilities |
| 006 | [moving-around.md](006-moving-around.md) | Navigating the file system |
| 007 | [thinking-in-nu.md](007-thinking-in-nu.md) | Thinking in data pipelines |
| 008 | [nushell-cheat-sheet.md](008-nushell-cheat-sheet.md) | Quick reference cheat sheet |

### Fundamentals

| # | File | Description |
|---|------|-------------|
| 009 | [nu-fundamentals.md](009-nu-fundamentals.md) | Core concepts and fundamentals |
| 010 | [types-of-data.md](010-types-of-data.md) | Data types in Nushell (strings, lists, records, tables) |
| 011 | [loading-data.md](011-loading-data.md) | Loading data from files and URLs |
| 012 | [pipelines.md](012-pipelines.md) | Working with pipelines |
| 013 | [working-with-strings.md](013-working-with-strings.md) | String manipulation |
| 014 | [working-with-lists.md](014-working-with-lists.md) | List operations |
| 015 | [custom-commands.md](015-custom-commands.md) | Creating custom commands |
| 016 | [working-with-records.md](016-working-with-records.md) | Record/object operations |
| 017 | [working-with-tables.md](017-working-with-tables.md) | Table operations |
| 018 | [navigating-structured-data.md](018-navigating-structured-data.md) | Accessing nested data |
| 019 | [variables.md](019-variables.md) | Variables and scoping |

### Programming

| # | File | Description |
|---|------|-------------|
| 020 | [control-flow.md](020-control-flow.md) | If, match, loops |
| 021 | [configuration.md](021-configuration.md) | Configuring Nushell |
| 022 | [environment.md](022-environment.md) | Environment variables |
| 023 | [modules.md](023-modules.md) | Creating and using modules |
| 024 | [coming-from-bash.md](024-coming-from-bash.md) | Migration guide from Bash |
| 025 | [aliases.md](025-aliases.md) | Creating aliases |
| 026 | [scripts.md](026-scripts.md) | Writing scripts |
| 027 | [running-externals.md](027-running-externals.md) | Running external commands |

### Advanced Topics

| # | File | Description |
|---|------|-------------|
| 028 | [plugins.md](028-plugins.md) | Extending Nushell with plugins |
| 029 | [parallelism.md](029-parallelism.md) | Parallel processing with par-each |
| 030 | [dataframes.md](030-dataframes.md) | High-performance dataframes with Polars |
| 031 | [explore.md](031-explore.md) | Interactive table pager |
| 032 | [creating-errors.md](032-creating-errors.md) | Custom error messages |
| 033 | [overlays.md](033-overlays.md) | Layered definitions (like virtual environments) |
| 034 | [sorting.md](034-sorting.md) | Sorting data (basic, custom, natural) |
| 035 | [testing.md](035-testing.md) | Writing tests for Nushell code |
| 036 | [style-guide.md](036-style-guide.md) | Best practices and coding conventions |

### Shell Features

| # | File | Description |
|---|------|-------------|
| 037 | [line-editor.md](037-line-editor.md) | Reedline - Nushell's line editor |
| 038 | [hooks.md](038-hooks.md) | Running code at specific events |
| 039 | [background-jobs.md](039-background-jobs.md) | Background job management |
| 040 | [custom-completions.md](040-custom-completions.md) | Custom tab completions |
| 041 | [coloring-theming.md](041-coloring-theming.md) | Customizing colors and themes |
| 042 | [coming-from-cmd.md](042-coming-from-cmd.md) | Migration guide from CMD.EXE |
| 043 | [coming-from-powershell.md](043-coming-from-powershell.md) | Migration guide from PowerShell |
| 044 | [stdout-stderr-exit-codes.md](044-stdout-stderr-exit-codes.md) | Working with stdout, stderr, exit codes |
| 045 | [directory-stack.md](045-directory-stack.md) | Directory stack for quick navigation |
| 046 | [externs.md](046-externs.md) | Defining signatures for external commands |
| 047 | [3rd-party-prompts.md](047-3rd-party-prompts.md) | Using Starship, oh-my-posh |

### Reference

| # | File | Description |
|---|------|-------------|
| 048 | [metadata.md](048-metadata.md) | Understanding metadata/tags |
| 049 | [standard-library.md](049-standard-library.md) | Built-in standard library |
| 050 | [nushell-map.md](050-nushell-map.md) | Command map (vs SQL, PowerShell, Bash) |
| 051 | [nushell-map-imperative.md](051-nushell-map-imperative.md) | Map vs Python, Kotlin, C++, Rust |
| 052 | [nushell-map-functional.md](052-nushell-map-functional.md) | Map vs Clojure, OCaml, Haskell |
| 053 | [nushell-operator-map.md](053-nushell-operator-map.md) | Operator reference |

---

## How to Use This Book

### For Beginners
Start with chapters 001-019 to understand fundamentals, then move to programming topics (020-027).

### For Intermediate Users
Focus on chapters 028-041 for advanced features like plugins, dataframes, and customization.

### For Advanced Users
Check out the reference chapters (048-053) for deep dives and migration guides.

### Quick Reference
- **Cheat Sheet**: See [008-nushell-cheat-sheet.md](008-nushell-cheat-sheet.md)
- **Coming from Bash**: See [024-coming-from-bash.md](024-coming-from-bash.md)
- **Coming from PowerShell**: See [043-coming-from-powershell.md](043-coming-from-powershell.md)
- **Operator Reference**: See [053-nushell-operator-map.md](053-nushell-operator-map.md)

---

## Related Resources

- [Official Nushell Book](https://www.nushell.sh/book/)
- [Nushell Commands Reference](https://www.nushell.sh/commands/)
- [Nushell Cookbook](https://www.nushell.sh/cookbook/)
- [Nushell GitHub](https://github.com/nushell/nushell)

---

*Last updated: 2024 | nubook - Nushell Book Collection*
