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
        use std::time::Instant;

        fn print_result<T: Display>(func: impl FnOnce(&str) -> Option<T>, input: &str) {
            let timer = Instant::now();
            let result = func(input);
            let elapsed = timer.elapsed();

            match result {
                Some(value) => println!("{} (took {:.2?})", value, elapsed),
                None => println!("(not solved)"),
            }
        }

        print!("Part {}: ", $part_number);
        print_result($part_fn, $input);
    }};
}
