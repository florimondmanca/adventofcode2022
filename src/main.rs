use std::{
    env,
    process::{self, Command},
};

fn main() {
    (1..=21).for_each(|day| {
        let target = format!("{day:02}");

        let mut args = vec!["run", "--bin", &target, "--"];

        if !env::args().any(|s| s == "--include-slow") {
            args.push("--skip-slow");
        }

        println!("------");
        println!("Day {day:02}");
        println!("------");

        let cmd = Command::new("cargo").args(&args).output().unwrap();

        let exit_code = match cmd.status.code() {
            Some(0) => 0,
            Some(code) => {
                println!("Error: target {target:?} exited with code {code}");
                1
            }
            None => panic!(),
        };

        println!("Stdout:\n{}", &String::from_utf8(cmd.stdout).unwrap());

        if exit_code > 0 {
            println!("Stderr:\n{}", &String::from_utf8(cmd.stderr).unwrap());
            process::exit(exit_code);
        }
    });
}
