use std::env::args;
use std::io::Read;
use std::process::exit;
use std::str::FromStr;
use std::time::{Duration, Instant};

pub fn run(year: i32, days: [[fn(String); 2]; 25]) {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
    }

    let (day_s, part_s) = args[1].split_once(':').unwrap_or_else(|| print_usage(&args[0]));

    let day = i32::from_str(day_s).unwrap_or_else(|_| print_usage(&args[0]));
    let part = i32::from_str(part_s).unwrap_or_else(|_| print_usage(&args[0]));

    if day < 1 || day > 25 || part < 1 || part > 2 {
        eprintln!("day must be between 1-25 and part must be 1 or 2");
        print_usage(&args[0]);
    }

    let mut text: String;
    if args.len() > 2 && args[2] == "real" {
        let mut aoc = libaoc::AocClient::new_from_env();
        text = aoc.get_input(year, day).unwrap_or_else(|_| {
            eprintln!("failed to retrieve input text");
            exit(2);
        });
    } else {
        println!("Enter your puzzle input, ending with Ctrl-D (EOF): (use 'aoc23 <day>:<part> real' to automatically download your real data)");
        text = String::new();
        std::io::stdin()
            .read_to_string(&mut text)
            .expect("Failed to read input from stdin!");
        println!("\n");
    }

    let before = Instant::now();
    days[day as usize - 1][part as usize - 1](text);
    let time = Instant::now().duration_since(before);
    let formatted_time = if time < Duration::from_secs(10) {
        format!("Completed in {:.1}ms", time.as_secs_f64() / 1000.0)
    } else {
        format!("Completed in {:.3}s", time.as_secs_f64())
    };
    println!(
        "{}{}",
        formatted_time,
        if cfg!(debug_assertions) {
            " (debug build)"
        } else {
            ""
        }
    );
}

fn print_usage(bin_name: &str) -> ! {
    eprintln!("Usage: {bin_name} <day>:<part> [real]");
    exit(1);
}
