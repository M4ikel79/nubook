# Creating Your Own Errors

Using metadata information, you can create custom error messages. Error messages have multiple parts:

- The title of the error
- The label of the error message (text + span to underline)

## Creating Errors

Use the `error make` command to create custom errors:

```nu
let span = (metadata $x).span;
error make {
    msg: "this is fishy",
    label: {
        text: "fish right here",
        span: $span
    }
}
```

## Example Custom Command

```nu
def my-command [x] {
    let span = (metadata $x).span;
    error make {
        msg: "this is fishy",
        label: {
            text: "fish right here",
            span: $span
        }
    }
}

my-command 100
# => Error:
# =>   × this is fishy
# =>    ╭─[entry #5:1:1]
# =>  1 │ my-command 100
# =>    ·            ─┬─
# =>    ·             ╰── fish right here
# =>    ╰────
```

The span captures where the argument came from, allowing you to point users to the exact location of the problem.
