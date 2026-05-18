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

fn choose_hostname(base_time: std::time::Instant) -> String {
    let suits = ["swords", "wands", "pentacles", "cups"];
    let minor = [
        "ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "page",
        "knight", "queen", "king",
    ];
    let major = [
        "fool",
        "magician",
        "high-priestess",
        "empress",
        "emperor",
        "hierophant",
        "lovers",
        "chariot",
        "strength",
        "hermit",
        "wheel",
        "justice",
        "hanged-man",
        "death",
        "temperance",
        "devil",
        "tower",
        "star",
        "moon",
        "sun",
        "judgment",
        "world",
    ];

    let now = base_time.elapsed().subsec_nanos();
    let mut ix = (now as usize) % 78;
    if ix < major.len() {
        format!("{:05x}-{}", now, major[ix])
    } else {
        ix -= major.len();
        format!(
            "{:05x}-{}-{}",
            now,
            minor[ix % minor.len()],
            suits[ix / minor.len()]
        )
    }
}

// -c: command or entrypoint to run in the container
// -m : Mount/Root Fs
// -u : User id the container will run as

fn main() {
    // set base_time. If I was using libc, I would use clock_gettime(CLOCK_MONOTONIC)
    // And reference for base_time would be time since boot.

    let base_time = std::time::Instant::now();

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

    // Check linux version
    // Choose hostname
    child_config.hostname = choose_hostname(base_time);
}
