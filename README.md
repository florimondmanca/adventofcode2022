# adventofcode2022

Rust code solutions to [Advent Of Code 2022](https://adventofcode.com/2022/).

## Requirements

* [Rust](https://www.rust-lang.org/) (used: 1.62.0)

## Usage

```
make
```

## Index

| Concept or technique | Days | Notes |
|---|---|---|
| Basics | 1 | |
| Text processing | all | |
| `Vec` | all | |
| `HashSet` | 3, 9 | |
| `HashMap` | 7, 8, 12 | |
| `VecDeque` | 11, 12 | |
| Range | 5, 8, 9 | |
| Struct | 4, 5 | |
| Integer maths | 9 | `abs()`, `signum()`, `%` |
| Type aliases | 5 | |
| Sort | 1, 11 | |
| `char` to ASCII code | 3, 12 | E.g. `'a' as u32` |
| Chunking w/ `.tuples()` | 3 | Uses `itertools` |
| Closures (`|| -> (...)`) | 10 | |
| `Fn`: Closure or function as parameter | 5, 11 | SO discussion : [How do you pass a Rust function as a parameter?](https://stackoverflow.com/questions/36390665/how-do-you-pass-a-rust-function-as-a-parameter) |
| `FnMut`: Mutable closures of functions as parameter | 10 | |
| Function generics | 5, 10, 11 | |
| Function as attribute: `Box<dyn Fn...>` | 11 | |
| `Vec` of `Vec` manipulation | 5 | |
| Circular references w/ `Rc` and `RefCell` | 7 | Full discussion: [Rust data structures with circular references](https://eli.thegreenplace.net/2021/rust-data-structures-with-circular-references/) |
| Interior mutability w/ `RefCell` | 11 | The Book: [`RefCell<T>` and the interior mutability pattern](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) |

## License

MIT
