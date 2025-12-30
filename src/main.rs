use std::env;
// use std::path::PathBuf;
// use std::path::Path;
use std::io::{self, Write, BufRead};
use std::process::Command;

fn main() {
    let stdin = io::stdin();

    loop {
        print_prompt();
        let mut line = String::new();
        let bytes_read = stdin.lock().read_line(&mut line);
        match bytes_read {
            Ok(0) => break,     // EOF
            Ok(_) => {
                let command = line.trim();

                // "ls -l -a" => ["ls", "-l", "-a"]
                let parts: Vec<&str> = command.split_whitespace().collect();
                execute_command(parts);
            }
            Err(e) => {
                eprintln!("Error on read: {:?}", e);
                break;
            }
        }
    }
}

fn print_prompt() {
    let current_dir = env::current_dir();
    let path = match current_dir {
        Ok(dir) => {
            // println!("Current directory: {:?}", dir);
            dir
        }
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };

    let basename = path.file_name();
    let basename_str = match basename {
        Some(name) => name.to_str().unwrap_or("/"),
        None => "/",
    };
    print!("[rustsh {}]$ ", basename_str);
    io::stdout().flush().unwrap();
}

fn execute_command(parts: Vec<&str>) {
    if parts.is_empty() {
        return;
    }

    let program_name = parts[0];
    let program_args = &parts[1..];
    let argc = program_args.len();

    // built-in command cd & exit
    if program_name == "cd" {
        // handle cd
        if argc != 1 {
            eprintln!("Error: invalid command. `cd` only takes 1 argument.");
            return;
        }

        // check if the path is valid
        // enter new dir
        if let Err(_) = env::set_current_dir(program_args[0]) {
            eprintln!("Error: invalid directory");
            return;
        }

        return;
    }

    if program_name == "exit" {
        // handle exit
        if argc != 0 {
            eprintln!("Error: invalid command. `argc` takes no argument.");
            return;
        }
        std::process::exit(0);
    }

    let program = if program_name.contains("/") {
        program_name.to_string()
    } else {
        // format!("/usr/bin/{}", command)      for linux
        format!("/bin/{}", program_name)     // for mac
    };

    // .status() Executes a command as a child process, waiting for it to finish and collecting its status.
    if let Err(_) = Command::new(program)
                            .args(program_args)
                            .status() {
        eprintln!("Error: invalid program");
    }
}