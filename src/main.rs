use std::env;
use std::io::stdout;
use std::io::{self, Write};
//Dependencies
use chrono::prelude::*;

fn main() {
    if cfg!(windows) {
        run_repl("windows");
    } else if cfg!(unix) {
        println!("UNIX not implemented yet.");
    }
}

fn run_repl(platform: &str) {
    print!("\x1B[2J\x1B[1;1H");
    println!("{} repl.", platform);

    loop {
        let mut input = String::new();
        let current_directory = std::env::current_dir().unwrap();
        print!("{}", current_directory.display());
        flush();
        flush_print("> ");
        std::io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        // println!("{}", command);

        if platform == "windows" {
            match &command as &str {
                "exit" => break,
                "clear" => print!("\x1B[2J\x1B[1;1H"),
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = std::path::Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                }
                "ls" => println!("{}", cmd("dir")),
                "cwd" => println!("{}", current_directory.display()),
                "date" => println!("{}", Local::now().format("%Y-%m-%d %H:%M:%S")),
                "tasks" => println!("{}", cmd("tasklist")),
                "del" => println!("{}", cmd_with_args("del", args.collect::<Vec<&str>>()[0])),
                "touch" => println!(
                    "{}",
                    cmd_with_args("type nul >", args.collect::<Vec<&str>>()[0])
                ),
                "cat" => println!("{}", cmd_with_args("type", args.collect::<Vec<&str>>()[0])),
                unknown => cmd_and_state(unknown),
            };
        } else if platform == "unix" {
            match &command as &str {
                "exit" => break,
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = std::path::Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                }
                "cwd" => println!("{}", current_directory.display()),
                "date" => println!("{}", Local::now().format("%Y-%m-%d %H:%M:%S")),
                "tasks" => println!("{}", cmd("tasklist")),
                "tree" => println!("{}", cmd("ls -R")),
                unknown => cmd_and_state(unknown),
            };
        }
    }
}

fn cmd(command: &str) -> String {
    let output = std::process::Command::new("cmd")
        .args(&["/C", command])
        .output()
        .expect("INTERNAL ERROR: failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}

fn cmd_with_args(command: &str, args: &str) -> String {
    let output = std::process::Command::new("cmd")
        .args(&["/C", command, args])
        .output()
        .expect("INTERNAL ERROR: failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}

fn cmd_and_state(command: &str) /*-> String*/
{
    // let date = Utc::now().format("%Y-%m-%d %H:%M:%S"); .split(" ").nth(0).unwrap()
    println!(
        "Execution at {}. Details:",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    let _output = std::process::Command::new("cmd")
        .args(&["/C", command])
        .status()
        .expect("INTERNAL ERROR: failed to execute process");
    // output.to_string()
}

fn flush() {
    stdout().flush().unwrap();
}

fn flush_print(value: &str) {
    print!("{}", value);
    flush();
}
