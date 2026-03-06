# Nushell Cheat Sheet

## Data Types

convert string to integer:
```nu
"12" | into int
```

convert present date to provided time zone:
```nu
date now | date to-timezone "Europe/London"
```

update a record's language and if none is specified insert provided value:
```nu
{'name': 'nu', 'stars': 5, 'language': 'Python'} | upsert language 'Rust'
```

convert list of strings to yaml:
```nu
[one two three] | to yaml
```

print table data:
```nu
[[framework, language]; [Django, Python] [Laravel, PHP]]
```

select two named columns from the table and print their values:
```nu
[{name: 'Robert' age: 34 position: 'Designer'}
 {name: 'Margaret' age: 30 position: 'Software Developer'}
 {name: 'Natalie' age: 50 position: 'Accountant'}
] | select name position
```

## Strings

interpolate text:
```nu
let name = "Alice"
$"greetings, ($name)!"
# => greetings, Alice!
```

split text on comma delimiter:
```nu
let string_list = "one,two,three" | split row ","
```

check if a string contains a substring:
```nu
"Hello, world!" | str contains "o, w"
# => true
```

join multiple strings with delimiter:
```nu
let str_list = [zero one two]
$str_list | str join ','
# => zero,one,two
```

slice text by indices:
```nu
'Hello World!' | str substring 4..8
# => o Wor
```

parse string into named columns:
```nu
'Nushell 0.80' | parse '{shell} {version}'
# => ╭───┬─────────┬─────────╮
# => │ # │  shell  │ version │
# => ├───┼─────────┼─────────┤
# => │ 0 │ Nushell │ 0.80    │
# => ╰───┴─────────┴─────────╯
```

parse comma separated values (csv):
```nu
"acronym,long\nAPL,A Programming Language" | from csv
```

color text in command-line terminal:
```nu
$'(ansi purple_bold)This text is a bold purple!(ansi reset)'
```

## Lists

insert list value at index:
```nu
[foo bar baz] | insert 1 'beeze'
# => ╭───┬───────╮
# => │ 0 │ foo   │
# => │ 1 │ beeze │
# => │ 2 │ bar   │
# => │ 3 │ baz   │
# => ╰───┴───────╯
```

update list value by index:
```nu
[1, 2, 3, 4] | update 1 10
# => ╭───┬────╮
# => │ 0 │  1 │
# => │ 1 │ 10 │
# => │ 2 │  3 │
# => │ 3 │  4 │
# => ╰───┴────╯
```

prepend list value:
```nu
let numbers = [1, 2, 3]
$numbers | prepend 0
```

append list value:
```nu
let numbers = [1, 2, 3]
$numbers | append 4
```

slice first list values:
```nu
[cammomile marigold rose forget-me-not] | first 2
```

iterate over a list:
```nu
let planets = [Mercury Venus Earth Mars Jupiter Saturn Uranus Neptune]
$planets | each { |elt| $"($elt) is a planet of the solar system" }
```

iterate over a list with an index and value:
```nu
$planets | enumerate | each { |elt| $"($elt.index + 1) - ($elt.item)" }
```

reduce the list to a single value:
```nu
let scores = [3 8 4]
$"total = ($scores | reduce { |elt, acc| $acc + $elt })"
# => total = 15
```

reduce with an initial value (`--fold`):
```nu
let scores = [3 8 4]
$"total = ($scores | reduce --fold 1 { |elt, acc| $acc * $elt })"
# => total = 96
```

give access to the 3rd item in the list:
```nu
let planets = [Mercury Venus Earth Mars Jupiter Saturn Uranus Neptune]
$planets.2
# => Earth
```

check if any string in the list starts with `E`:
```nu
let planets = [Mercury Venus Earth Mars Jupiter Saturn Uranus Neptune]
$planets | any {|elt| $elt | str starts-with "E" }
# => true
```

slice items that satisfy provided condition:
```nu
let cond = {|x| $x < 0 }; [-1 -2 9 1] | take while $cond
```

## Tables

sort table:
```nu
ls | sort-by size
```

sort table, get first rows:
```nu
ls | sort-by size | first 5
```

concatenate two tables with same columns:
```nu
let $a = [[first_column second_column third_column]; [foo bar snooze]]
let $b = [[first_column second_column third_column]; [hex seeze feeze]]
$a | append $b
```

remove the last column of a table:
```nu
let teams_scores = [[team score plays]; ['Boston Celtics' 311 3] ['Golden State Warriors', 245 2]]
$teams_scores | drop column
```

## Files and Filesystem

open a text file with the default text editor:
```nu
start file.txt
```

save a string to text file:
```nu
'lorem ipsum ' | save file.txt
```

append a string to the end of a text file:
```nu
'dolor sit amet' | save --append file.txt
```

save a record to file.json:
```nu
{ a: 1, b: 2 } | save file.json
```

recursively search for files by file name:
```nu
glob **/*.{rs,toml} --depth 2
```

watch a file, run command whenever it changes:
```nu
watch . --glob=**/*.rs {|| cargo test }
```

## Custom Commands

custom command with parameter type set to string:
```nu
def greet [name: string] {
    $"hello ($name)"
}
```

custom command with default parameter set to nushell:
```nu
def greet [name = "nushell"] {
    $"hello ($name)"
}
```

passing named parameter by defining flag for custom commands:
```nu
def greet [
    name: string
    --age: int
] {
    [$name $age]
}

greet world --age 10
```

using flag as a switch:
```nu
def greet [
    name: string
    --age (-a): int
    --twice
] {
    if $twice {
        [$name $age $name $age]
    } else {
        [$name $age]
    }
}
greet -a 10 --twice hello
```

custom command which takes any number of positional arguments using rest params:
```nu
def greet [...name: string] {
    print "hello all:"
    for $n in $name {
        print $n
    }
}
greet earth mars jupiter venus
```

## Variables

an immutable variable cannot change its value after declaration:
```nu
let val = 42
print $val
# => 42
```

shadowing variable (declaring variable with the same name in a different scope):
```nu
let val = 42
do { let val = 101;  $val }
# => 101
$val
# => 42
```

declaring a mutable variable with mut key word:
```nu
mut val = 42
$val += 27
$val
# => 69
```

closures and nested defs cannot capture mutable variables from their environment:
```nu
mut x = 0
[1 2 3] | each { $x += 1 }
# => Error: Capture of mutable variable.
```

a constant variable is immutable and is fully evaluated at parse-time:
```nu
const file = 'path/to/file.nu'
source $file
```

use question mark operator `?` to return null instead of error:
```nu
let files = (ls)
$files.name?.0?
```

assign the result of a pipeline to a variable:
```nu
let big_files = (ls | where size > 10kb)
$big_files
```

## Modules

use an inline module:
```nu
module greetings {
    export def hello [name: string] {
        $"hello ($name)!"
    }

    export def hi [where: string] {
        $"hi ($where)!"
    }
}
use greetings hello
hello "world"
```

import module from file and use its environment in current scope:
```nu
# greetings.nu
export-env {
    $env.MYNAME = "Arthur, King of the Britons"
}
export def hello [] {
    $"hello ($env.MYNAME)"
}

use greetings.nu
$env.MYNAME
# => Arthur, King of the Britons
greetings hello
```

use main command in module:
```nu
# greetings.nu
export def hello [name: string] {
    $"hello ($name)!"
}

export def main [] {
    "greetings and salutations!"
}

use greetings.nu
greetings
# => greetings and salutations!
```
