use std::{env, fs};

pub fn read_file(directory: &str, day: u32) -> String {
    let cwd = env::current_dir().unwrap();

    let path = cwd
        .join("src")
        .join(directory)
        .join(format!("{day:02}.txt"));

    let file = fs::read_to_string(path);

    file.expect("File does not exist")
}

#[macro_export]
macro_rules! solve {
    ($part_number:expr, $part_fn:ident, $input:expr) => {{
        use std::fmt::Display;

        fn print_result<T: Display>(func: impl FnOnce(&str) -> Option<T>, input: &str) {
            print!("Part {}: ", $part_number);
            match func(input) {
                Some(value) => println!("{}", value),
                None => println!("(not solved)"),
            }
        }

        print_result($part_fn, $input);
    }};
}

#[macro_export]
macro_rules! slow {
    ($body: expr) => {{
        use std::env;

        let skip_slow = env::args().any(|flag| flag == "--skip-slow");

        if skip_slow {
            println!("SKIPPED (slow)");
        } else {
            $body();
        }
    }};
}
