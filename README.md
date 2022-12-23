# adventofcode2022

Rust code solutions to [Advent Of Code 2022](https://adventofcode.com/2022/).

## Requirements

* [Rust](https://www.rust-lang.org/) (used: 1.62.0)

## Usage

Run puzzles using:

```
make
```

Some puzzles take a long time to solve. They are ignored by default. To solve them too, use:

```
make everything
```

Run a specific day:

```
make one DAY=03
```

## Index

| Concept or technique | Days | Notes |
|---|---|---|
| Basics | 1 | |
| Text processing | all | |
| `Vec` | all | |
| `[T; N]` (fixed-size array) | 18, 19, 22, 23 | |
| `HashSet` | 3, 6, 8, 9, 16, 17, 18 | |
| `HashMap` | 7, 8, 11, 12, 14, 16, 21, 23 | |
| `VecDeque` | 11, 12, 18 | Queue implementation |
| `BinaryHeap` | 16, 19 | Heap implementation (queue that keeps items sorted) |
| `Vec<Vec<_>>` manipulation (e.g. grids) | 5, 22, 23 | |
| Collection indexing | 7, 8, 11, 12, 16, 19, 23 | `vec[idx]`, `map[idx]`, etc |
| Range (`a..b`)) | 5, 8, 9, 14, 18 | |
| RangeInclusive (`a..=b`) | 19, 23 | |
| Struct | 4, 5, ... | |
| Enum | 13, 23 | |
| Module constants | 19 | |
| `loop { ... } ` | 6, 14, 17 | |
| `match` | 7, 9, 11, 13, 14, 17, 19, 21, 22, 23 | |
| `Some` / `None` | 7, 12, 13, 14 | |
| `while let Some(...) = ...` | 11, 12, 18 | |
| Integer maths | 9 | `abs()`, `signum()`, `%` |
| Type aliases | 5, ... | |
| Sorting | 1, 11, 13, 14 | |
| `Iterator::position()` | 20, 22 | |
| `char` to ASCII code | 3, 12 | E.g. `'a' as u32` |
| Advanced formatting | 14, 17 | |
| Regular expressions | 15, 16, 19 | Uses the `regex` crate |
| Chunking w/ `.tuples()` | 3 | Uses the `itertools` crate |
| Closures (`|| -> (...)`) | 10 | |
| `Fn` | 11 | Closure as an argument. SO discussion : [How do you pass a Rust function as a parameter?](https://stackoverflow.com/questions/36390665/how-do-you-pass-a-rust-function-as-a-parameter) |
| `FnMut` | 10 | Mutable closure as argument |
| `fn func<T>(...)` | 10, 11 | Function generics |
| `struct S<'a>` | 11 | Structs with ref properties (e.g. `name: &'a str`) require defining a lifetime |
| `RefCell` | 7, 11 | An implementation of the [interior mutability pattern]([`RefCell<T>` and the interior mutability pattern](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)) |
| `Rc` | 7 | A solution for circular references (e.g. trees). Full discussion: [Rust data structures with circular references](https://eli.thegreenplace.net/2021/rust-data-structures-with-circular-references/) |
| `impl Ord` and `impl PartialOrd` | 13, 16, 19 | Allows custom sorting. See [page in the Book](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html#partialord-and-ord-for-ordering-comparisons) | 
| `impl From<T>` | 19 | Custom `::from(...)` implementation. |
| Breadth-first search (BFS) | 12, 18 | |
| Depth-first search (DFS) | 19 | |

## License

MIT
