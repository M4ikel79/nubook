# Working with Strings

As with most languages, strings are a collection of 0 or more characters that represent text. This can include file names, file paths, names of columns, and much more. Strings are so common that Nushell offers multiple string formats to match your use-case:

## String Formats at a Glance

| Format | Example | Notes |
|--------|---------|-------|
| Single-quoted | `'[^\n]+'` | Cannot contain single quotes within the string |
| Double-quoted | `"The\nEnd"` | C-style backslash escapes |
| Raw strings | `r#'Raw string'#` | May include single quotes |
| Bare word | `ozymandias` | Can only contain "word" characters |
| Backtick | `` `[^\n]+` `` | Can include whitespace |
| Single-quoted interpolation | `$'Captain ($name)'` | Cannot contain any `'` or unmatched `()` |
| Double-quoted interpolation | `$"Captain ($name)"` | C-style backslash escapes |

## Single-quoted Strings

The simplest string in Nushell is the single-quoted string. This string uses the `'` character to surround some text:

```nu
'hello world'
# => hello world
```

Single-quoted strings don't do anything to the text they're given, making them ideal for holding a wide range of text data.

## Double-quoted Strings

For more complex strings, Nushell also offers double-quoted strings. These support escape characters using the `\` character:

```nu
"hello\nworld"
# => hello
# => world
```

Nushell currently supports the following escape characters:
- `\"` - double-quote character
- `\'` - single-quote character
- `\\` - backslash
- `\/` - forward slash
- `\b` - backspace
- `\f` - formfeed
- `\r` - carriage return
- `\n` - newline
- `\t` - tab
- `\u{X...}` - a single unicode character

## Raw Strings

Raw strings behave the same as a single quoted strings, except that raw strings may also contain single quotes:

```nu
r#'Raw strings can contain 'quoted' text.'#
# => Raw strings can contain 'quoted' text.
```

Additional `#` symbols can be added to nest raw strings.

## Bare Word Strings

Strings consisting of a single 'word' can also be written without any quotes:

```nu
print hello
# => hello
```

But be careful - on the command line it will be interpreted as an external command:

```nu
hello
# => Error: executable was not found
```

## Backtick-quoted Strings

Backtick-quoted strings using the `` ` `` character can include spaces:

```nu
`ls`
`..`
`./my dir`
```

## Appending and Prepending to strings

```nu
['foo', 'bar'] | each {|s| '~/' ++ $s} # ~/foo, ~/bar

['foo', 'bar'] | str replace -r '^' '~/' # ~/foo, ~/bar
['foo', 'bar'] | str replace -r '$' '~/' # foo~/, bar~/

"hello" | append "world!" | str join " " # hello world!
```

## String interpolation

String interpolation combines the results together using `$" "` and `$' '`:

```nu
let name = "Alice"
$"greetings, ($name)"
# => greetings, Alice
```

By wrapping expressions in `()`, we can run them to completion and use the results to help build the string.

String interpolation supports escaping parentheses:

```nu
$"2 + 2 is (2 + 2) \(you guessed it!)"
# => 2 + 2 is 4 (you guessed it!)
```

## Splitting Strings

```nu
"red,green,blue" | split row ","
# => [red, green, blue]

"red,green,blue" | split column ","
# => table with columns

'aeiou' | split chars
# => [a, e, i, o, u]
```

## The `str` command

Many string functions are subcommands of the `str` command.

### Trimming Strings

```nu
'       My   string   ' | str trim
# => My   string

'=== Nu shell ===' | str trim -r -c '='
# => === Nu shell
```

### Substrings

```nu
'Hello World!' | str index-of 'o'
# => 4

'Hello World!' | str substring 4..8
# => o Wo
```

### String Padding

```nu
'1234' | fill -a right -c '0' -w 10
# => 0000001234
```

### Reversing Strings

```nu
'Nushell' | str reverse
# => llehsuN
```

## String Parsing

```nu
'Nushell 0.80' | parse '{shell} {version}'
# => table with shell and version columns

'where all data is structured!' | parse --regex '(?P<subject>\w*\s?\w+) is (?P<adjective>\w+)'
```

## String Comparison

```nu
'APL' =~ '^\w{0,3}$'
# => true

'JavaScript' starts-with 'Java'
# => true

'OCaml' ends-with 'Caml'
# => true
```

## Converting Strings

### To string

```nu
123 | into string
# or
$'<(123)>'
```

### From string

```nu
'123' | into int
```

## Coloring Strings

```nu
$'(ansi purple_bold)This text is a bold purple!(ansi reset)'
```

`ansi purple_bold` makes the text bold purple, `ansi reset` resets the coloring.
