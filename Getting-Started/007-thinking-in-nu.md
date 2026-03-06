# Thinking in Nu

Nushell is different! It's common (and expected!) for new users to have some existing "habits" or mental models coming from other shells or languages.

The most common questions from new users typically fall into one of the following topics:

-   Nushell isn't Bash
-   Implicit Return
-   Single Return Value per Expression
-   Every Command Returns a Value
-   Think of Nushell as a Compiled Language
-   Variables are Immutable by Default
-   Nushell's Environment is Scoped

## Nushell isn't Bash

Nushell is both a programming language and a shell. Because of this, it has its own way of working with files, directories, websites, and more. You'll find that some features in Nushell work similar to those you're familiar with in other shells.

For example, the following commandline works the same in both Bash and Nushell on Unix/Linux platforms:

```nu
curl -s https://api.github.com/repos/nushell/nushell/contributors | jq -c '.[] | {login,contributions}'
```

Nushell has many other similarities with Bash (and other shells) and many commands in common.

> **Tips**
> Bash is primarily a command interpreter which runs external commands. Nushell provides many of these as cross-platform, built-in commands.

> **Thinking in Nushell**
> Nushell borrows concepts from many shells and languages. You'll likely find many of Nushell's features familiar.

### But it's not Bash

Because of this, however, it's sometimes easy to forget that some Bash (and POSIX in general) style constructs just won't work in Nushell. For instance, in Bash, it would be normal to write:

```sh
# Redirect using >
echo "hello" > output.txt
```

In Nushell, however, the `>` is used as the greater-than operator for comparisons.

Since `>` is an operator, redirection to a file in Nushell is handled through a pipeline command that is dedicated to saving content - `save`:

```nu
"hello" | save output.txt
```

## Implicit Return

Users coming from other shells will likely be very familiar with the `echo` command. Nushell's `echo` might appear the same at first, but it is *very* different.

First, notice how the following output *looks* the same in both Bash and Nushell:

```nu
echo "Hello, World"
# => Hello, World
```

But while the other shells are sending `Hello, World` straight to *standard output*, Nushell's `echo` is simply *returning a value*. Nushell then *renders* the return value of a command.

> **Thinking in Nushell**
> Most anywhere you might write `echo <something>`, in Nushell, you can just write `<something>` instead.

## Single Return Value per Expression

It's important to understand that an expression can only return a single value. If there are multiple subexpressions inside an expression, only the ***last*** value is returned.

A common mistake is to write a custom command definition like this:

```nu
def latest-file [] {
    echo "Returning the last file"
    ls | sort-by modified | last
}
```

The *output* of that pipeline becomes the *return value* of the `latest-file` custom command.

## Every Command Returns a Value

Some languages have the concept of "statements" which don't return values. Nushell does not.

In Nushell, ***every command returns a value***, even if that value is `null` (the `nothing` type).

## Think of Nushell as a Compiled Language

In Nushell, there are exactly two, separate, high-level stages when running code:

1.  *Stage 1 (Parser):* Parse the ***entire*** source code
2.  *Stage 2 (Engine):* Evaluate the ***entire*** source code

It can be useful to think of Nushell's parsing stage as *compilation* in static languages like Rust or C++.

> **Important**
> However, this also means that Nushell cannot currently support an `eval` construct as with *dynamic* languages such as Bash or Python.

### Features Built on Static Parsing

On the other hand, the ***static*** results of Parsing are key to many features of Nushell and its REPL, such as:

-   Accurate and expressive error messages
-   Semantic analysis for earlier and robust detection of error conditions
-   IDE integration
-   The type system
-   The module system
-   Completions
-   Custom command argument parsing
-   Syntax highlighting
-   Real-time error highlighting
-   Profiling and debugging commands

### Limitations

The static nature of Nushell often leads to confusion for users coming to Nushell from languages where an `eval` is available.

> **Note**
> For a more in-depth explanation of this section, see [How Nushell Code Gets Run](/book/how_nushell_code_gets_run.html).

## Variables are Immutable by Default

Another common surprise when coming from other languages is that Nushell variables are immutable by default. While Nushell has optional mutable variables, many of Nushell's commands are based on a functional-style of programming which requires immutability.

Immutable variables are also key to Nushell's `par-each` command, which allows you to operate on multiple values in parallel using threads.

> **Thinking in Nushell**
> If you're used to relying on mutable variables, it may take some time to relearn how to code in a more functional style.

## Nushell's Environment is Scoped

Nushell takes multiple design cues from compiled languages. One such cue is that languages should avoid global mutable state. Shells have commonly used global mutation to update the environment, but Nushell attempts to steer clear of this approach.

In Nushell, blocks control their own environment. Changes to the environment are scoped to the block where they occur.

> **Thinking in Nushell**
> Use scoped-environment to write more concise scripts and prevent unnecessary or unwanted global environment mutation.
