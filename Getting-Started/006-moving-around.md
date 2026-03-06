# Moving Around the System

A defining characteristic of a shell is the ability to navigate and interact with the filesystem. Nushell is, of course, no exception. Here are some common commands you might use when interacting with the filesystem:

## Viewing Directory Contents

```nu
ls
```

As seen in the Quick Tour, the `ls` command returns the contents of a directory. Nushell's `ls` will return the contents as a table.

The `ls` command also takes an optional argument to change what you'd like to view. For example, we can list the files that end in ".md"

```nu
ls *.md
# => ╭───┬────────────────────┬──────┬──────────┬──────────────╮
# => │ # │        name        │ type │   size   │   modified   │
# => ├───┼────────────────────┼──────┼──────────┼──────────────┤
# => │ 0 │ CODE_OF_CONDUCT.md │ file │  3.4 KiB │ 9 months ago │
# => │ 1 │ CONTRIBUTING.md    │ file │ 11.0 KiB │ 5 months ago │
# => │ 2 │ README.md          │ file │ 12.0 KiB │ 6 days ago   │
# => │ 3 │ SECURITY.md        │ file │  2.6 KiB │ 2 months ago │
# => ╰───┴────────────────────┴──────┴──────────┴──────────────╯
```

## Glob Patterns (wildcards)

The asterisk (`*`) in the above optional argument `*.md` is sometimes called a wildcard or a glob. It lets us match anything. You can read this glob `*.md` as *"match any filename, so long as it ends with '.md'."*

The most general glob is `*`, which will match all paths. More often, you'll see this pattern used as part of another pattern, for example `*.bak` and `temp*`.

Nushell also supports a double `*` which will traverse paths that are nested inside of other directories. For example, `ls **/*` will list all the non-hidden paths nested under the current directory.

```nu
ls **/*.md
# => ╭───┬───────────────────────────────┬──────┬──────────┬──────────────╮
# => │ # │             name              │ type │   size   │   modified   │
# => ├───┼───────────────────────────────┼──────┼──────────┼──────────────┤
# => │ 0 │ CODE_OF_CONDUCT.md            │ file │  3.4 KiB │ 5 months ago │
# => │ 1 │ CONTRIBUTING.md               │ file │ 11.0 KiB │ a month ago  │
# => │ 2 │ README.md                     │ file │ 12.0 KiB │ a month ago  │
# => │ 3 │ SECURITY.md                   │ file │  2.6 KiB │ 5 hours ago  │
# => │ 4 │ benches/README.md             │ file │    249 B │ 2 months ago │
# => │ 5 │ crates/README.md              │ file │    795 B │ 5 months ago │
# => │ 6 │ crates/nu-cli/README.md       │ file │    388 B │ 5 hours ago  │
# => │ 7 │ crates/nu-cmd-base/README.md  │ file │    262 B │ 5 hours ago  │
# => │ 8 │ crates/nu-cmd-extra/README.md │ file │    669 B │ 2 months ago │
# => │ 9 │ crates/nu-cmd-lang/README.md  │ file │   1.5 KiB │ a month ago  │
# => ╰───┴───────────────────────────────┴──────┴──────────┴──────────────╯
```

Here, we're looking for any file that ends with ".md". The double-asterisks further specify *"in any directory starting from here."*

Nushell's globbing syntax not only supports `*`, but also matching single characters with `?` and character groups with `[...]`.

Escaping the `*`, `?`, and `[]` patterns works by enclosing them in a single-quoted, double-quoted, or raw string. For example, to show the contents of a directory named `[slug]`, use `ls "[slug]"` or `ls '[slug]'`.

However, *backtick* quoted strings do not escape globs.

> **Tips**
> Nushell also includes a dedicated `glob` command with support for more complex globbing scenarios.

### Converting Strings to Globs

The quoting techniques above are useful when constructing glob-literals, but you may need to construct globs programmatically. There are several techniques available for this purpose:

1.  `into glob` - The `into glob` command can be used to convert a string (and other types) into a glob.
2.  The spread operator combined with the `glob` command
3.  Force `glob` type via annotation

## Creating a Directory

As with most other shells, the `mkdir` command is used to create new directories. One subtle difference is that Nushell's internal `mkdir` command operates like the Unix/Linux `mkdir -p` by default.

> **Tips**
> A common mistake when coming to Nushell is to attempt to use `mkdir -p <directory>` as in the native Linux/Unix version. However, this will generate an `Unknown Flag` error on Nushell.

## Changing the Current Directory

```nu
cd cookbook
```

To change from the current directory to a new one, use the `cd` command.

Changing the current working directory can also be done if `cd` is omitted and a path by itself is given:

```nu
cookbook/
```

Just as in other shells, you can use either the name of the directory, or if you want to go up a directory you can use the `..` shortcut.

You can also add additional dots to go up additional directory levels:

```nu
# Change to the parent directory
cd ..
# or
..
# Go up two levels (parent's parent)
cd ...
# or
...
# Go up three levels (parent of parent's parent)
cd ....
```

> **Tips**
> Multi-dot shortcuts are available to both internal Nushell filesystem commands as well as to external commands.

> **IMPORTANT TIP**
> Changing the directory with `cd` changes the `PWD` environment variable. This means that a change of a directory is kept to the current scope (e.g. block or closure). Once you exit the block, you'll return to the previous directory.

## Filesystem Commands

Nu also provides some basic filesystem commands that work cross-platform such as:

-   `mv` to rename or move a file or directory to a new location
-   `cp` to copy an item to a new location
-   `rm` to remove items from the filesystem

> **Note**
> Under Bash and many other shells, most filesystem commands (other than `cd`) are actually separate binaries in the system. In Nushell, these commands are built-in. This has several advantages.

While the use of the Nushell built-in versions is typically recommended, it is possible to access the Linux binaries. See [Running System Commands](/book/running_externals.html) for details.
