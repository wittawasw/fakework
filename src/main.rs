use rand::{rng, Rng};
use std::{thread, time};
use chrono::Local;
use colored::*;

const COMMANDS: &[&str] = &[
    "Analyzing data streams",
    "Compiling resources",
    "Running diagnostic checks",
    "Optimizing performance metrics",
    "Decrypting secure partitions",
    "Validating encryption keys",
    "Synchronizing with remote server",
    "Checking system logs",
    "Indexing database records",
];

const WARNINGS: &[&str] = &[
    "Memory usage exceeded threshold",
    "Unusual network activity detected",
    "Packet loss detected in secure channel",
    "Performance bottleneck identified",
];

const ERRORS: &[(&str, &str)] = &[
    ("Unable to establish a secure connection", "Connection established successfully"),
    ("Corrupted database index found", "Database re-indexed successfully"),
    ("Indexing Error: Out of Memory", "Indexing resumed with optimized memory usage"),
];

fn print_with_timestamp(level: &str, message: &str, color: Color) {
    let now = Local::now();
    let timestamp = format!("[{}]", now.format("%Y-%m-%d %H:%M:%S")).dimmed();
    let log_type = match level {
        "INFO" => "[INFO]".green(),
        "WARNING" => "[WARNING]".yellow(),
        "ERROR" => "[ERROR]".red(),
        _ => "[LOG]".white(),
    };
    println!("{} {} {}", timestamp, log_type, message.color(color));
}

fn main() {
    let mut rng = rng();
    let mut last_failed_task: Option<(&str, &str)> = None;

    loop {
        if let Some((failed_task, success_message)) = last_failed_task {
            print_with_timestamp("INFO", &format!("Retrying failed task: {}...", failed_task), Color::Green);
            thread::sleep(time::Duration::from_secs(rng.random_range(2..5)));

            // 50% chance of retry success
            if rng.random_range(0..2) == 0 {
                print_with_timestamp("INFO", &format!("    └─ {}", success_message), Color::Green);
                last_failed_task = None;
            } else {
                print_with_timestamp("ERROR", &format!("{} failed again.", failed_task), Color::Red);
                continue;
            }
        } else {
            let choice = rng.random_range(0..10);

            match choice {
                0..=5 => {
                    let command = COMMANDS[rng.random_range(0..COMMANDS.len())];
                    print_with_timestamp("INFO", &format!("{}...", command), Color::Green);
                    print_with_timestamp("INFO", &format!("    ├─ Processing batch {}", rng.random_range(1000..9999)), Color::White);
                    print_with_timestamp("INFO", &format!("    ├─ Loaded {} records from cache", rng.random_range(50..500)), Color::White);
                    print_with_timestamp("INFO", &format!("    └─ Operation completed in {}ms", rng.random_range(20..150)), Color::White);
                }
                6..=8 => {
                    let warning = WARNINGS[rng.random_range(0..WARNINGS.len())];
                    print_with_timestamp("WARNING", warning, Color::Yellow);
                    print_with_timestamp("WARNING", &format!("    ├─ Retrying in {}s", rng.random_range(3..10)), Color::White);
                    print_with_timestamp("WARNING", "    └─ System monitoring engaged", Color::White);
                }
                9 => {
                    let (error, success_message) = ERRORS[rng.random_range(0..ERRORS.len())];
                    print_with_timestamp("ERROR", error, Color::Red);

                    let retries = rng.random_range(1..=3); // Up to 3 retries
                    for attempt in 1..=retries {
                        let wait_time = rng.random_range(2..6);
                        print_with_timestamp("ERROR", &format!("    ├─ Attempt {}: Retrying in {}s", attempt, wait_time), Color::White);
                        thread::sleep(time::Duration::from_secs(wait_time));

                        if attempt == retries {
                            if rng.random_range(0..2) == 0 {
                                print_with_timestamp("ERROR", "    ├─ Forced process restart initiated...", Color::Red);
                                print_with_timestamp("ERROR", "    └─ Re-attempting the same task.", Color::Red);
                                last_failed_task = Some((error, success_message));
                            } else {
                                print_with_timestamp("INFO", "    ├─ Forced process restart initiated...", Color::Green);
                                print_with_timestamp("INFO", &format!("    └─ {}", success_message), Color::Green);
                            }
                            break;
                        }
                    }
                }
                _ => (),
            }
        }
        thread::sleep(time::Duration::from_secs(rng.random_range(1..=5)));
    }
}
