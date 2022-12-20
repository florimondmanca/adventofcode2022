# adventofcode2022

Rust code solutions to [Advent Of Code 2022](https://adventofcode.com/2022/).

## Requirements

* [Rust](https://www.rust-lang.org/) (used: 1.62.0)

## Usage

Compute and show solutions using:

```
make
```

Some solutions take a long time to compute, so they're skipped by default. To compute them too, use:

```
make all-slow
```

## Index

| Concept or technique | Days | Notes |
|---|---|---|
| Basics | 1 | |
| Text processing | all | |
| `Vec` | all | |
| `HashSet` | 3, 9, 18 | |
| `HashMap` | 7, 8, 12, 14 | |
| `VecDeque` | 11, 12, 18 | |
| Range | 5, 8, 9, 14, 18 | |
| Struct | 4, 5, ... | |
| Enum | 13 | |
| `loop { ... } ` | 6, 14, 17 | |
| `match` | 7, 9, 11, 13, 14, 17 | |
| `Some` / `None` | 7, 12, 13, 14 | |
| `while let Some(...) = ...` | 11, 12, 18 | |
| Integer maths | 9 | `abs()`, `signum()`, `%` |
| Type aliases | 5, ... | |
| Sort | 1, 11, 13, 14 | |
| `char` to ASCII code | 3, 12 | E.g. `'a' as u32` |
| Advanced formatting | 14, 17 | |
| Chunking w/ `.tuples()` | 3 | Uses `itertools` |
| Closures (`|| -> (...)`) | 10 | |
| `Fn`: Closure or function as parameter | 5, 11 | SO discussion : [How do you pass a Rust function as a parameter?](https://stackoverflow.com/questions/36390665/how-do-you-pass-a-rust-function-as-a-parameter) |
| `FnMut`: Mutable closures of functions as parameter | 10 | |
| Function generics | 5, 10, 11 | |
| Function as attribute: `Box<dyn Fn...>` | 11 | |
| `Vec` of `Vec` manipulation | 5 | |
| Circular references w/ `Rc` and `RefCell` | 7 | Full discussion: [Rust data structures with circular references](https://eli.thegreenplace.net/2021/rust-data-structures-with-circular-references/) |
| Interior mutability w/ `RefCell` | 11 | The Book: [`RefCell<T>` and the interior mutability pattern](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) |
| Custom ordering w/ `Ord` and `PartialOrd` traits | 13 | See [page in the Book](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html#partialord-and-ord-for-ordering-comparisons) | 
| Breadth-first search (BFS) | 12, 18 | |

## License

MIT
