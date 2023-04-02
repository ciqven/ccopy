use std::io;
use std::env;
use std::process;
use std::thread;
use std::time;

use arboard::Clipboard;

fn main() {
    // Grab stdin
    let data = consume_stdin();

    // Calculate duration
    let args: Vec<String> = env::args().collect();
    let dur: time::Duration;

    if args.len() < 2 {
        dur = time::Duration::from_secs(5);
    } else {
        dur = time::Duration::from_secs(args[1].parse().unwrap_or(5));
    }

    // Set as clipboard content
    let mut clipboard = Clipboard::new().expect("Could not create clipboard");
    clipboard.set_text(data).expect("Could not set clipboard contents");
    // Sleep to keep clipboard provider active (good job X11...)
    thread::sleep(dur);
}

fn consume_stdin() -> String {
    let mut result = String::new();

    loop {
        let mut buffer = String::new();

        let read_chars = io::stdin().read_line(&mut buffer).unwrap_or_else(|error| {
            eprintln!("Error reading stdin: \"{:?}\"", error);
            process::exit(1);
        });

        if read_chars < 1 {
            break;
        }

        result.push_str(&buffer);
    }

    return result;
}