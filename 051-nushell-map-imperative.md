# Nu Map from Imperative Languages

Mapping Nushell to Python, Kotlin, C++, Rust.

## Collection Operations

| Nushell | Python | Kotlin | C++ | Rust |
|---------|--------|--------|-----|------|
| `append` | `list.append` | `add` | `push_back` | `push` |
| `prepend` | `deque.appendleft` | - | - | - |
| `insert` | `dict[key]=val` | `map.insert` | `map.insert` | - |
| `drop` | `list[:-3]` | - | - | - |
| `first` | `list[:x]` | `List[0]` | `vector[0]` | `Vec[0]` |
| `last` | `list[-x:]` | - | `&Vec[len-1]` | - |
| `take` | `list[:x]` | `&Vec[..x]` | - | - |
| `skip` | `list[x:]` | `&Vec[x..]` | - | - |
| `get` | `list[x]` | `List[x]` | `vector[x]` | `Vec[x]` |
| `get` | `dict["key"]` | `Map["key"]` | `map["key"]` | `HashMap[key]` |
| `length` | `len` | `size` | `length` | `len` |
| `is-empty` | `is None` | `isEmpty` | `empty` | `is_empty` |
| `reverse` | `reversed` | `reverse` | `reverse` | `rev` |
| `shuffle` | `random.shuffle` | - | - | - |
| `uniq` | `set` | Set | `set` | `HashSet` |

## Data Processing

| Nushell | Python | Kotlin | C++ | Rust |
|---------|--------|--------|-----|------|
| `each` / `for` | `for` | `for` | `for` | `for` |
| `where` | `filter` | `filter` | `filter` | `filter` |
| `reduce` | `functools.reduce` | `reduce` | `reduce` | `fold` |
| `group-by` | `itertools.groupby` | `groupBy` | - | - |
| `sort-by` | `sorted` | `sortedBy` | `sort` | `sort` |
| `select` | `{k:dict[k] for k}` | - | - | - |
| `merge` | `dict.append` | `map.extend` | - | - |
| `transpose` | `zip(*matrix)` | - | - | - |
| `reject` | `del` | - | - | - |

## Strings

| Nushell | Python | Kotlin | C++ | Rust |
|---------|--------|--------|-----|------|
| `lines` | `splitlines` | `split` | `views::split` | `split` |
| `str join` | `str.join` | `joinToString` | `join` | - |
| `str trim` | `strip` | `trim` | Regex | `trim` |
| `split row` | `str.split` | `split` | `views::split` | `split` |
| `format` | `format` | `format` | `format` | `format!` |

## Files/System

| Nushell | Python | Kotlin | C++ | Rust |
|---------|--------|--------|-----|------|
| `open` | `open` | - | - | - |
| `save` | `write` | - | - | - |
| `touch` | `open(path,'a').close()` | - | - | - |
| `ls` | `os.listdir` | `fs::read_dir` | - | - |
| `mkdir` | `os.mkdir` | `fs::create_dir` | - | - |
| `cp` | `shutil.copy` | `fs::copy` | - | - |
| `mv` | `shutil.move` | `fs::rename` | - | - |
| `rm` | `os.remove` | - | - | - |
| `du` | `shutil.disk_usage` | - | - | - |

## Other

| Nushell | Python | Kotlin | C++ | Rust |
|---------|--------|--------|-----|------|
| `match` | `match` | `when` | `match` | `match` |
| `date` | `datetime.date.today` | `LocalDate.now` | - | - |
| `exit` | `exit()` | `System.exit` | `exit` | `exit` |
| `kill` | `os.kill` | - | - | - |
| `pwd` | `os.getcwd` | `env::current_dir` | - | - |
| `range` | `range` | `..`, `until` | `iota` | `..` |
| `version` | `sys.version` | - | - | - |
| `$env` | `os.environ` | - | - | - |
| `http get` | `urllib.request` | - | - | - |
| `which` | `shutil.which` | - | - | - |
