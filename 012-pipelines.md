# Pipelines

One of the core designs of Nu is the pipeline, a design idea that traces its roots back decades to some of the original philosophy behind Unix. Just as Nu extends from the single string data type of Unix, Nu also extends the idea of the pipeline to include more than just text.

## Basics

A pipeline is composed of three parts: the input, the filter, and the output.

```nu
open Cargo.toml | update workspace.dependencies.base64 0.24.2 | save Cargo_new.toml
```

The first command, `open Cargo.toml`, is an **input** (sometimes also called a "source" or "producer"). This creates or loads data and feeds it into a pipeline.

The second command, `update workspace.dependencies.base64 0.24.2`, is a **filter**. Filters take the data they are given and often do something with it.

The last command, `save Cargo_new.toml`, is an **output** (sometimes called a "sink"). An output takes input from the pipeline and does some final operation on it.

The `$in` variable will collect the pipeline into a value for you, allowing you to access the whole stream as a parameter:

```nu
[1 2 3] | $in.1 * $in.2
# => 6
```

## Multi-line pipelines

If a pipeline is getting a bit long for one line, you can enclose it within parentheses `()`:

```nu
let year = (
    "01/22/2021" |
    parse "{month}/{day}/{year}" |
    get year
)
```

## Semicolons

Take this example:

```nu
line1; line2 | line3
```

Here, semicolons are used in conjunction with pipelines. When a semicolon is used, no output data is produced to be piped. As such, the `$in` variable will not work when used immediately after the semicolon.

## Pipeline Input and the Special `$in` Variable

Much of Nu's composability comes from the special `$in` variable, which holds the current pipeline input.

`$in` is particularly useful when used in:
- Command or external parameters
- Filters
- Custom command definitions or scripts that accept pipeline input

### `$in` as a Command Argument

```nu
mkdir $'((date now) + 1day | format date '%F') Report'
```

or using pipelines:

```nu
date now                    # 1: today
| $in + 1day                # 2: tomorrow
| format date '%F'          # 3: Format as YYYY-MM-DD
| $'($in) Report'           # 4: Format the directory name
| mkdir $in                 # 5: Create the directory
```

### Pipeline Input in Filter Closures

Certain filter commands may modify the pipeline input to their closure:

```nu
1..10 | each {$in * 2}
```

### When Does `$in` Change?

- **Rule 1:** When used in the first position of a pipeline in a closure or block, `$in` refers to the pipeline input to the closure/block.
- **Rule 2:** When used anywhere else in a pipeline (other than the first position), `$in` refers to the previous expression's result.
- **Rule 3:** When used with no input, `$in` is null.
- **Rule 4:** In a multi-statement line separated by semicolons, `$in` cannot be used to capture the results of the previous statement.

## Working with External Commands

Nu commands communicate with each other using the Nu data types, but what about commands outside of Nu?

`internal_command | external_command` - Data will flow from the internal_command to the external_command. This data will get converted to a string.

`external_command | internal_command` - Data coming from an external command into Nu will come in as bytes that Nushell will try to automatically convert to UTF-8 text.

`external_command_1 | external_command_2` - Nu works with data piped between two external commands in the same way as other shells.

### Command Input and Output Types

You can check what a command supports with `help <command name>`, which shows the relevant Input/output types.

## Rendering Display Results

In interactive mode, when a pipeline ends, the `display_output` hook configuration defines how the result will be displayed. The default configuration uses the `table` command to render structured data.

```nu
$env.config.hooks.display_output = { table -e }
[1,2,3,[4,5,6]]
# => Shows expanded table
```

## Output Result to External Commands

Sometimes you want to output Nushell structured data to an external command for further processing. However, Nushell's default formatting options may not be what you want. You must explicitly convert the data to a string before piping it to an external:

```nu
ls /usr/share/nvim/runtime/ | get name | to text | ^grep tutor
```

## Command Output in Nushell

Unlike external commands, Nushell commands are akin to functions. Most Nushell commands do not print anything to `stdout` and instead just return data.

```nu
do { ls; ls; ls; "What?!" }
```

This will only display `"What?!"` because that is the value returned by the `do` command. However, using the system `^ls` command instead of `ls` would print the directory thrice because `^ls` does print its result.
