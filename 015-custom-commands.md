# Custom Commands

As with any programming language, you'll quickly want to save longer pipelines and expressions so that you can call them again easily when needed. This is where custom commands come in.

> **Note**
> Custom commands are similar to functions in many languages, but in Nushell, custom commands *act as first-class commands themselves*.

## Creating and Running a Custom Command

Let's start with a simple `greet` custom command:

```nu
def greet [name] {
  $"Hello, ($name)!"
}
```

To run this command:

```nu
greet "World"
# => Hello, World!
```

## Returning Values from Commands

Nushell features an *implicit return*, where the value of the final expression in the command becomes its return value.

```nu
def eight [] {
  1 + 1
  2 + 2
  4 + 4
}

eight
# => 8
```

Early return:

```nu
def process-list [] {
  let input_length = length
  if $input_length > 10_000 {
    print "Input list is too long"
    return null
  }
  $in | each {|i| $i * 4.25 }
}
```

Suppressing the return value:

```nu
def create-three-files [] {
  [ file1 file2 file3 ] | each {|filename|
    touch $filename
  } | ignore
}
```

## Custom Commands and Pipelines

### Pipeline Output

```nu
def my-ls [] { ls }
my-ls | get name
```

### Pipeline Input

Custom commands can also take input from the pipeline:

```nu
def double [] {
  each { |num| 2 * $num }
}

[1 2 3] | double
# => [2, 4, 6]
```

We can also store the input for later using `$in`:

```nu
def nullify [...cols] {
  let start = $in
  $cols | reduce --fold $start { |col, table|
    $table | upsert $col null
  }
}

ls | nullify name size
```

## Naming Commands

In Nushell, a command name can be a string of characters. It's common practice to separate words with `-`.

```nu
def "custom command" [] {
  "This is a custom command with a space in the name!"
}
```

## Parameters

### Multiple parameters

```nu
def greet [name1 name2] {
  $"Hello, ($name1) and ($name2)!"
}
```

### Required positional parameters

```nu
def greet [name1, name2] {
  $"Hello, ($name1) and ($name2)!"
}

greet Wei Mei
# => Hello, Wei and Mei!
```

### Optional Positional Parameters

```nu
def greet [name?: string] {
  $"Hello, ($name | default 'You')"
}

greet
# => Hello, You
```

### Parameter Types

```nu
def greet [name: string] {
  $"Hello, ($name)"
}

def congratulate [age: int = 18] {
  $"Happy birthday! You are ($age) years old now!"
}
```

### Flags

```nu
def greet [
  name: string
  --age: int
] {
  { name: $name, age: $age }
}

greet Lucia --age 23
greet --age 39 Ali
```

Shorthand flags:

```nu
def greet [
  name: string
  --age (-a): int
] { ... }

greet Akosua -a 35
```

Switch flags:

```nu
def greet [
  name: string
  --caps
] {
  let greeting = $"Hello, ($name)!"
  if $caps { $greeting | str upcase } else { $greeting }
}

greet Miguel --caps
# => HELLO, MIGUEL!
```

### Rest parameters

```nu
def multi-greet [...names: string] {
  for $name in $names {
    print $"Hello, ($name)!"
  }
}

multi-greet Elin Lars Erik
```

### Rest Parameters with Wrapped External Commands

```nu
def --wrapped ezal [...rest] {
  eza -l ...$rest
}

ezal commands
```

## Pipeline Input-Output Signature

```nu
def "str stats" []: string -> record { }
```

```nu
def "str join" [separator?: string]: [
  list -> string
  string -> string
] { }
```

## Documenting Your Command

```nu
# Greet guests along with a VIP
def vip-greet [
  vip: string        # The special guest
   ...names: string  # The other guests
] {
  for $name in $names {
    print $"Hello, ($name)!"
  }
  print $"And a special welcome to our VIP today, ($vip)!"
}
```

## Adding Attributes to Custom Commands

```nu
@example "Greet a VIP" {vip-greet "Bob"}
@example "Greet multiple people" {vip-greet "Bob" ["Alice" "Charlie"]}
def vip-greet [...]
```

```nu
@deprecated "Use vip-greet as a replacement."
@category "deprecated"
def greet [...]
```

## Changing the Environment in a Custom Command

```nu
def --env foo [] {
  $env.FOO = 'After'
}

$env.FOO = "Before"
foo
$env.FOO
# => After
```

### Changing Directories

```nu
def --env go-home [] {
  cd ~
}

cd /
go-home
pwd
# => Your home directory
```

## Persisting

To make custom commands available in future Nushell sessions, add them to your startup configuration:
- Directly in your `config.nu`
- To a file sourced by your `config.nu`
- To a module imported by your `config.nu`
