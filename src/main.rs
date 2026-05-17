use std::process::{self};

#[derive(Debug)]
struct ChildConfig {
    argc: usize,
    uid: u32,
    fd: i32,
    hostname: String,
    argv: Vec<String>,
    mount_dir: String,
}

// -c: command or entrypoint to run in the container
// -m : Mount/Root Fs
// -u : User id the container will run as

fn main() {
    let mut child_config = ChildConfig {
        argc: 0,
        uid: 0,
        fd: 0,
        hostname: String::new(),
        argv: Vec::new(),
        mount_dir: String::new(),
    };

    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-c" => {
                // consume the rest of the arguments as the command to run in the container
                child_config.argv.extend(args);
                break;
            }
            "-m" => {
                let mount_dir = args.next().unwrap_or_else(|| {
                    eprintln!("Error: -m requires a mount directory argument");
                    process::exit(1);
                });
                child_config.mount_dir = mount_dir;
            }
            "-u" => {
                let uid_str = args.next().unwrap_or_else(|| {
                    eprintln!("Error: -u requires a user ID argument");
                    process::exit(1);
                });
                child_config.uid = uid_str.parse().unwrap_or_else(|_| {
                    eprintln!("Error: Invalid user ID: {}", uid_str);
                    process::exit(1);
                });
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                process::exit(1);
            }
        }
    }

    child_config.argc = child_config.argv.len();

    if child_config.argv.is_empty() {
        eprintln!("Error: -c <command> is required.");
        process::exit(1);
    }

    if child_config.mount_dir.is_empty() {
        eprintln!("Error: -m <mount_dir> is required.");
        process::exit(1);
    }
    // print arguments
    println!("child_config: {:?}", child_config);
}
