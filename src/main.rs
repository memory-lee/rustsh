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

                if !command.is_empty() {
                    let program = if command.contains('/') {
                        command.to_string()
                    } else {
                        // format!("/usr/bin/{}", command)  for linux
                        format!("/bin/{}", command)     // for mac
                    };

                    // .status() Executes a command as a child process, waiting for it to finish and collecting its status.
                    if let Err(_) = Command::new(program).status() {
                        eprintln!("Error: invalid program");
                    }

                }
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
    print!("[nyush {}]$ ", basename_str);
    io::stdout().flush().unwrap();
}