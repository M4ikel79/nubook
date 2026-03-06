# Parallelism

Nushell has early support for running code in parallel using commands with `par-` naming. These correspond to non-parallel versions, allowing you to easily convert serial scripts to parallel ones.

## par-each

The most common parallel command is `par-each`, a companion to `each`.

Like `each`, `par-each` works on each element in the pipeline, running a block on each. Unlike `each`, `par-each` does these operations in parallel.

### Example: Counting files in subdirectories

Using `each`:
```nu
ls | where type == dir | each { |row|
    { name: $row.name, len: (ls $row.name | length) }
}
```
This took 21ms on the test machine.

Using `par-each`:
```nu
ls | where type == dir | par-each { |row|
    { name: $row.name, len: (ls $row.name | length) }
}
```
This now runs in 6ms - quite a difference!

### Environment Variables

Because environment variables are scoped, you can use `par-each` to work in multiple directories in parallel:
```nu
ls | where type == dir | par-each { |row|
    { name: $row.name, len: (cd $row.name; ls | length) }
}
```

### Important Notes

Results may come back in different orders each run (depending on hardware threads). If you need results in a particular order, add sorting steps:
```nu
ls | where type == dir | par-each { |row|
    { name: $row.name, len: (ls $row.name | length) }
} | sort-by name
```
