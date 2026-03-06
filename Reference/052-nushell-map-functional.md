# Nu Map from Functional Languages

Mapping Nushell to Clojure, OCaml/Elm, Haskell.

## Collection Operations

| Nushell | Clojure | Tablecloth | Haskell |
|---------|---------|------------|---------|
| `append` | conj, into | append, (++) | (++) |
| `prepend` | cons | cons, :: | :: |
| `first` | first | head | head |
| `last` | last, peek | last | last |
| `take` | take | take, init | take, init |
| `skip` | rest | tail | tail |
| `length` | count | length | length |
| `reverse` | reverse | reverse | reverse |
| `uniq` | set | Set.empty | Data.Set |
| `is-empty` | empty? | isEmpty | - |
| `shuffle` | shuffle | - | - |

## Data Processing

| Nushell | Clojure | Tablecloth | Haskell |
|---------|---------|------------|---------|
| `each` | map, mapv | map, forEach | map |
| `where` | filter, filterv | filter, filterMap | filter |
| `reduce` | reduce | foldr | foldr |
| `group-by` | group-by | group, groupBy | - |
| `sort-by` | sort, sort-by | sort, sortBy | sort, sortBy |
| `select` | nth | Array.get | lookup |
| `merge` | merge | - | - |

## Strings

| Nushell | Clojure | Tablecloth | Haskell |
|---------|---------|------------|---------|
| `lines` | lines, words | split, words | split, words |
| `str join` | join | concat | intercalate |
| `str trim` | trim, triml | trim | strip |
| `split row` | split-* | split, lines | split, lines |

## Other

| Nushell | Clojure | Tablecloth | Haskell |
|---------|---------|------------|---------|
| `match` | match | case | case |
| `format` | format | printf | - |
| `date` | java.time.LocalDate | - | - |
| `open` | with-open | - | - |
| `slice` / `range` | range | range | 1..10 |
