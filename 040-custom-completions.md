# Custom Completions

Custom completions combine custom commands and completions for positional/flag parameters.

## Basic Custom Completion

Define a completion command that returns suggestions, then attach with `@`:

```nu
def animals [] { ["cat", "dog", "eel"] }
def my-command [animal: string@animals] { print $animal }
my-command<Tab>
# => cat   dog   eel
```

The `string@animals` syntax means: use `string` shape and `animals` completer.

Return empty list to suppress completions: `[]`

## Completion Options

Return a record for control over filtering/sorting:

```nu
def animals [] {
    {
        options: {
            case_sensitive: false
            completion_algorithm: substring
            sort: false
        }
        completions: [cat, rat, bat]
    }
}
```

- `sort`: Disable sorting
- `case_sensitive`: Override case sensitivity
- `completion_algorithm`: `prefix`, `substring`, or `fuzzy`

## Modules and Completions

Define completions in modules, export only commands:

```nu
module commands {
    def animals [] { ["cat", "dog"] }
    export def my-command [animal: string@animals] { }
}
```

Completers are attached at parse time - re-import module for changes.

## Context-Aware Completions

Pass context to completion command:

```nu
module commands {
    def animals [] { ["cat", "dog"] }
    def animal-names [context: string] {
        match ($context | split words | last) {
            cat => ["Missy", "Phoebe"]
            dog => ["Luna", "Max"]
        }
    }
    export def my-command [
        animal: string@animals
        name: string@animal-names
    ] { }
}
```

`$context` contains command-line typed so far.

Get cursor position:
```nu
def completer [context:string, position:int] {}
```

## External Commands

Add completions to extern commands:

```nu
export extern "git push" [
    remote?: string@"nu-complete git remotes"
    refspec?: string@"nu-complete git branches"
]
```

## Custom Descriptions and Styles

Return records with value, description, style:

```nu
def my_commits [] {
    [
        { value: "5c2464", description: "Add .gitignore", style: red }
        { value: "f3a377", description: "Initial commit", style: { fg: green, bg: "#66078c", attr: ub } }
    ]
}
```

Style formats:
- `"red"` - color name
- `{ fg: "green", bg: "blue", attr: b }` - record format
- JSON string of record

## External Completers

Set external completer in config:

```nu
$env.config.completions.external = {
    enable: true
    max_results: 100
    completer: $completer
}
```

Example with carapace:
```nu
let carapace_completer = {|spans|
    carapace $spans.0 nushell ...$spans | from json
}
```

Return `null` to fall back to file completion.
