# Quick Tour

## Nushell Commands Output *Data*

The easiest way to see what Nu can do is to start with some examples, so let's dive in.

The first thing you'll notice when you run a command like `ls` is that instead of a block of text coming back, you get a structured table.

```nu
ls
# => ╭────┬─────────────────────┬──────┬───────────┬──────────────╮
# => │  # │        name         │ type │   size    │   modified   │
# => ├────┼─────────────────────┼──────┼───────────┼──────────────┤
# => │  0 │ CITATION.cff        │ file │     812 B │ 2 months ago │
# => │  1 │ CODE_OF_CONDUCT.md  │ file │   3.4 KiB │ 9 months ago │
# => │  2 │ CONTRIBUTING.md     │ file │  11.0 KiB │ 5 months ago │
# => │  3 │ Cargo.lock          │ file │ 194.9 KiB │ 15 hours ago │
# => │  4 │ Cargo.toml          │ file │   9.2 KiB │ 15 hours ago │
# => │  5 │ Cross.toml          │ file │     666 B │ 6 months ago │
# => │  6 │ LICENSE             │ file │   1.1 KiB │ 9 months ago │
# => │  7 │ README.md           │ file │  12.0 KiB │ 15 hours ago │
# => │  8 │ ...             │ ...  │ ...       │ ...          │
# => ╰────┴─────────────────────┴──────┴───────────┴──────────────╯
```

This table does more than just format the output nicely. Like a spreadsheet, it allows us to work with the data *interactively*.

## Acting on Data

Next, let's sort this table by each file's size. To do this, we'll take the output from `ls` and feed it into a command that can sort tables based on the *values* in a column.

```nu
ls | sort-by size | reverse
# => ╭───┬─────────────────┬──────┬───────────┬──────────────╮
# => │ # │      name       │ type │   size    │   modified   │
# => ├───┼─────────────────┼──────┼───────────┼──────────────┤
# => │ 0 │ Cargo.lock      │ file │ 194.9 KiB │ 15 hours ago │
# => │ 1 │ toolkit.nu      │ file │  20.0 KiB │ 15 hours ago │
# => │ 2 │ README.md       │ file │  12.0 KiB │ 15 hours ago │
# => │ 3 │ CONTRIBUTING.md │ file │  11.0 KiB │ 5 months ago │
# => │ 4 │ ...             │ ...  │ ...       │ ...          │
# => │ 5 │ LICENSE         │ file │   1.1 KiB │ 9 months ago │
# => │ 6 │ CITATION.cff    │ file │     812 B │ 2 months ago │
# => │ 7 │ Cross.toml      │ file │     666 B │ 6 months ago │
# => │ 8 │ typos.toml      │ file │     513 B │ 2 months ago │
# => ╰───┴─────────────────┴──────┴───────────┴──────────────╯
```

Notice that we didn't pass commandline arguments or switches to `ls`. Instead, we used Nushell's built-in `sort-by` command to sort the *output* of the ls command. Then, to see the largest files on top, we used `reverse` on the *output* of sort-by.

Cool!

If you compare the sort order closely, you might realize that the data isn't sorted alphabetically. It's not even sorted by the *numerical* values. Instead, since the `size` column is a `filesize` type, Nushell knows that `1.1 KiB` (kibibytes) is larger than `812 B` (bytes).

## Finding Data Using the `where` Command

Nu provides many commands that can operate on the structured output of the previous command. These are usually categorized as "Filters" in Nushell.

For example, we can use `where` to filter the contents of the table so that it only shows files over 10 kilobytes:

```nu
ls | where size > 10kb
# => ╭───┬─────────────────┬──────┬───────────┬───────────────╮
# => │ # │      name       │ type │   size    │   modified    │
# => ├───┼─────────────────┼──────┼───────────┼───────────────┤
# => │ 0 │ CONTRIBUTING.md │ file │  11.0 KiB │ 5 months ago  │
# => │ 1 │ Cargo.lock      │ file │ 194.6 KiB │ 2 minutes ago │
# => │ 2 │ README.md       │ file │  12.0 KiB │ 16 hours ago  │
# => │ 3 │ toolkit.nu      │ file │  20.0 KiB │ 16 hours ago  │
# => ╰───┴─────────────────┴──────┴───────────┴───────────────╯
```

## More Than Just Directories

Of course, this isn't limited to the `ls` command. Nushell follows the Unix philosophy where each command does one thing well and you can typically expect the output of one command to become the input of another. This allows us to mix-and-match commands in many different combinations.

Let's look at a different command:

```nu
ps
# => ╭───┬──────┬──────┬───────────────┬──────────┬──────┬───────────┬─────────╮
# => │ # │ pid  │ ppid │     name      │  status  │ cpu  │    mem    │ virtual │
# => ├───┼──────┼──────┼───────────────┼──────────┼──────┼───────────┼─────────┤
# => │ 0 │    1 │    0 │ init(void)    │ Sleeping │ 0.00 │   1.2 MiB │ 2.2 MiB │
# => │ 1 │    8 │    1 │ init          │ Sleeping │ 0.00 │ 124.0 KiB │ 2.3 MiB │
# => │ 2 │ 6565 │    1 │ SessionLeader │ Sleeping │ 0.00 │ 108.0 KiB │ 2.2 MiB │
# => │ 3 │ 6566 │ 6565 │ Relay(6567)   │ Sleeping │ 0.00 │ 116.0 KiB │ 2.2 MiB │
# => │ 4 │ 6567 │ 6566 │ nu            │ Running  │ 0.00 │  28.4 MiB │ 1.1 GiB │
# => ╰───┴──────┴──────┴───────────────┼──────────┴──────┴───────────┴─────────╯
```

You may be familiar with the Linux/Unix `ps` command. It provides a list of all of the current processes running in the system along with their current status. As with `ls`, Nushell provides a cross-platform, built-in `ps` command that returns its results as structured data.

> **Note**
> The traditional Unix `ps` only shows the current process and its parents by default. Nushell's implementation shows all of the processes on the system by default.

Normally, running `ps` in Nushell uses its ***internal***, cross-platform command. However, it is still possible to run the ***external***, system-dependent version on Unix/Linux platforms by prefacing it with the *caret sigil*. For example:
```nu
^ps aux  # run the Unix ps command with all processes in user-oriented form
```

What if we wanted to just show the processes that are actively running? As with `ls` above, we can also work with the table that `ps` *outputs*:

```nu
ps | where status == Running
# => ╭───┬──────┬──────┬──────┬─────────┬──────┬──────────┬─────────╮
# => │ # │ pid  │ ppid │ name │ status  │ cpu  │   mem    │ virtual │
# => ├───┼──────┼──────┼──────┼─────────┼──────┼──────────┼─────────┤
# => │ 0 │ 6585 │ 6584 │ nu   │ Running │ 0.00 │ 31.9 MiB │ 1.2 GiB │
# => ╰───┴──────┴──────┴──────┴─────────┴──────┴──────────┴─────────╯
```

You can examine the types for the table's columns using:

```nu
ps | describe
# => table<pid: int, ppid: int, name: string, status: string, cpu: float, mem: filesize, virtual: filesize> (stream)
```

## Command Arguments in a Pipeline

Sometimes, a command takes an *argument* instead of pipeline *input*. For this scenario, Nushell provides the `$in` variable that let's you use the previous command's output in variable-form. For example:

```nu
ls
| sort-by size
| reverse
| first
| get name
| cp $in ~
```

> **Nushell Design Note**
> Whenever possible, Nushell commands are designed to act on pipeline *input*. However, some commands, like `cp` in this example, have two (or more) arguments with different meanings. In this case, `cp` needs to know both the path to *copy* as well as the *target* path. As a result, this command is more ergonomic with two *positional parameters*.

## Getting Help

Nushell provides an extensive, in-shell Help system. For example:

```nu
# help <command>
help ls
# Or
ls --help
# Also
help operators
help escapes
```

Press the F1 key to access the Help menu. Search for the `ps` command here, but *don't press Enter just yet*!

Instead, press the Down Arrow key, and notice that you are scrolling through the Examples section. Highlight an example, *then* press Enter and the example will be entered at the commandline, ready to run!

The Help system also has a "search" feature:

```nu
help --find filesize
# or
help -f filesize
```

It may not surprise you by now that the Help system itself is based on structured data! Notice that the output of `help -f filesize` is a table.

You can view *all* commands (other than externals) as a single large table using:

```nu
help commands
```

## `explore`'ing from Here

That `help commands` output is quite long. You could send it to a pager like `less` or `bat`, but Nushell includes a built-in `explore` command that lets you not only scroll, but also telescope-in to nested data. Try:

```nu
help commands | explore
```

Then press the Enter key to access the data itself. Use the arrow keys to scroll to the `cp` command, and over to the `params` column. Hit Enter again to telescope in to the complete list of parameters available to the `cp` command.

> **Note**
> Pressing Esc one time returns from Scroll-mode to the View; Pressing it a second time returns to the previous view (or exits, if already at the top view level).

You can, of course, use the `explore` command on *any* structured data in Nushell. This might include JSON data coming from a Web API, a spreadsheet or CSV file, YAML, or anything that can be represented as structured data in Nushell.

Try `$env.config | explore` for fun!
