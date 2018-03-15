use std::collections::HashMap;
use std::io;
use std::io::Write;

enum Command {
    End,
    Status,
    None,
}

struct Entry {
    job: String,
    time_logged: f32,
}

/**
 * Print a welcome message, with instructions.
 */
fn print_welcome_message() {
    println!("*------------------------------------*");
    println!("|             Rusty Time             |");
    println!("|                                    |");
    println!("| Format: [job]:[hours_spent]        |");
    println!("|                                    |");
    println!("| Commands:                          |");
    println!("|     end: Quit                      |");
    println!("|     status: Show time entries      |");
    println!("*------------------------------------*");
}

/**
 * Print the results of all time logged entries in the session. Also print a total time logged.
 */
fn show_status(entries: HashMap<String, f32>, total_time_logged: f32) {
    println!("");
    println!("*------------------------------------*");
    println!("|           Your Timesheet           |");
    println!("*------------------------------------*");
    println!("| Total time logged: {} hours", total_time_logged);
    println!("*------------------------------------*");
    
    for (key, value) in &entries {
        println!("{}: {} hours", key, value);
    }
}

fn validate_input(input: String) -> Result<Entry, &'static str> {
    if input.trim().is_empty() {
        Err("Empty string recorded. Skipping.")
    } else {
        let entry_parts: Vec<&str> = input.split(":").collect();

        if entry_parts.len() < 2 {
            Err("Invalid time entry. Please use [job]:[time_logged]. Skipping.")
        } else {
            let job: String = String::from(entry_parts[0]);
            let time_logged: f32 = match entry_parts[1].trim().parse() {
                Ok(num) => num,
                Err(_) => 0.0,
            };

            if time_logged == 0.0 {
                Err("Empty or invalid time logged. Skipping.")
            } else {
                Ok(Entry {
                    job: job,
                    time_logged: time_logged,
                })
            }
        }
    }
}

/**
 * Read a string from the user and save it.
 */
fn get_input() -> String {
    let mut input = String::new();

    // write a message, telling the user to give us some input
    print!(": ");
    io::stdout().flush().expect("Couldn't write to stdout.");

    // read the input and save it
    io::stdin().read_line(&mut input).expect("Could not read input.");

    // return the input string
    input
}

fn check_for_commands(input: String) -> Command {
    if input.trim() == "end" || input.trim() == "exit" {
        Command::End
    } else if input.trim() == "status" {
        Command::Status
    } else {
        Command::None
    }
}

fn main() {
    print_welcome_message();

    // start keeping track of time list, with empty dictionary
    let mut entries = HashMap::new();
    let mut total_time_logged = 0.0;

    loop {
        let input = get_input();

        match check_for_commands(input.clone()) {
            Command::End => {
                break;
            },
            Command::Status => {
                show_status(entries.clone(), total_time_logged);
                continue;
            },
            Command::None => {},
        }

        let entry = match validate_input(input.clone()) {
            Ok(en) => en,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        let existing_entry = entries.entry(entry.job).or_insert(0.0);
        *existing_entry += entry.time_logged;

        total_time_logged += entry.time_logged;
    }

    show_status(entries.clone(), total_time_logged);
}
