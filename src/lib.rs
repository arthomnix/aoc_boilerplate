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

    let text = if args.len() > 2 && (args[2] == "real" || args[2] == "example") {
        let mut aoc = libaoc::AocClient::new_from_env();
        if args[2] == "example" {
            let example = aoc.get_example(year, day, part).unwrap_or_else(|_| {
                eprintln!("failed to retrieve example input");
                exit(2);
            }).unwrap_or_else(|| {
                eprintln!("failed to parse example input");
                exit(2);
            });

            let expected_answer = if part == 2 {
                example.part2_answer
            } else {
                example.part1_answer
            };
            if let Some(answer) = expected_answer {
                println!("Expected answer (parsed from HTML, may be wrong!): {answer}");
            }

            if part == 2 {
                example.part2_data.unwrap_or(example.data)
            } else {
                example.data
            }
        } else {
            aoc.get_input(year, day).unwrap_or_else(|_| {
                eprintln!("failed to retrieve input text");
                exit(2);
            })
        }
    } else {
        println!("Enter your puzzle input, ending with Ctrl-D (EOF): (use '{} <day>:<part> [real|example]' to automatically download your real/example data)", &args[0]);
        let mut text = String::new();
        std::io::stdin()
            .read_to_string(&mut text)
            .expect("Failed to read input from stdin!");
        println!("\n");
        text
    };

    let benchmark_runs = std::env::var("AOC_BENCH_RUNS")
        .map(|x| x.parse::<i32>())
        .into_iter()
        .flatten()
        .next()
        .unwrap_or(1);

    let times = (0..benchmark_runs)
        .map(|_| {
            let t = text.clone();
            let before = Instant::now();
            days[day as usize - 1][part as usize - 1](t);
            before.elapsed()
        })
        .collect::<Vec<_>>();

    let time = times.iter().sum::<Duration>() / times.len() as u32;

    if benchmark_runs == 1 {
        println!(
            "Completed in {time:?}{}",
            if cfg!(debug_assertions) { " (debug build)" } else { "" },
        );
    } else {
        let min_duration = times.iter().min().unwrap();
        let max_duration = times.iter().max().unwrap();

        println!(
            "Completed in {time:?} ({benchmark_runs} runs; min = {min_duration:?}; max = {max_duration:?}){}",
             if cfg!(debug_assertions) { " (debug build)" } else { "" },
        );
    }
}

fn print_usage(bin_name: &str) -> ! {
    eprintln!("Usage: {bin_name} <day>:<part> [real|example]");
    eprintln!("The real/example options require you to provide your Advent of Code session token in the AOC_SESSION environment variable.");
    exit(1);
}
