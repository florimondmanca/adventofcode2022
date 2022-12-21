use std::{
    env,
    process::{self, Command},
};

fn main() {
    (19..20).for_each(|day| {
        let target = format!("{day:02}");

        let mut args = vec!["run", "--bin", &target, "--"];

        if !env::args().any(|s| s == "--include-slow") {
            args.push("--skip-slow");
        }

        println!("------");
        println!("Day {day:02}");
        println!("------");

        let cmd = Command::new("cargo").args(&args).output().unwrap();

        if let Some(code) = cmd.status.code() {
            if code != 0 {
                println!("Error: target {target:?} exited with code {code}");
                println!("Stdout: {}", &String::from_utf8(cmd.stdout).unwrap());
                println!("Stderr: {}", &String::from_utf8(cmd.stderr).unwrap());
                process::exit(1);
            }
        }

        let stdout = String::from_utf8(cmd.stdout).unwrap();
        println!("{stdout}");
    });
}
