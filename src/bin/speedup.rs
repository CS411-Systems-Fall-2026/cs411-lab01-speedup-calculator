//! Lab 1: The Speedup Calculator -- benchmark binary.
//!
//! Run: cargo run --bin speedup [--limit N]

use speedup::{amdahl_speedup, measure_sequential_fraction, parallel_sum_of_primes, sequential_sum_of_primes};
use std::env;
use std::io::{self, Write};
use std::time::Instant;

/// Prompt for the student's USI username; baked into the Success Token
/// so a copied/shared token decodes to someone else's name, not yours.
fn get_student_id() -> String {
    loop {
        print!("Enter your USI username (e.g. cwill): ");
        io::stdout().flush().ok();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        let trimmed = input.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
        println!("Username cannot be blank.");
    }
}

fn parse_limit_arg() -> u64 {
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        if args[i] == "--limit" && i + 1 < args.len() {
            if let Ok(v) = args[i + 1].parse::<u64>() {
                return v;
            }
        }
    }
    2_000_000
}

fn main() {
    let limit = parse_limit_arg();
    println!("Running speedup benchmark for limit = {}\n", limit);

    let start = Instant::now();
    let sequential_result = sequential_sum_of_primes(limit);
    let sequential_time = start.elapsed().as_secs_f64() * 1000.0;

    let s = measure_sequential_fraction(limit);
    println!("Estimated sequential fraction s = {:.4}\n", s);

    let thread_counts = [1usize, 2, 4, 8];
    println!(" Threads |  Time (ms) | Speedup | Amdahl Predicted");
    println!("---------+------------+---------+------------------");

    let mut measured_speedup_at_8: f64 = 0.0;
    let mut correctness_ok = true;

    for &n in &thread_counts {
        let t0 = Instant::now();
        let parallel_result = parallel_sum_of_primes(limit, n);
        let elapsed_ms = t0.elapsed().as_secs_f64() * 1000.0;

        if parallel_result != sequential_result {
            correctness_ok = false;
            eprintln!(
                "MISMATCH at {} threads: sequential={} parallel={}",
                n, sequential_result, parallel_result
            );
        }

        let measured_speedup = sequential_time / elapsed_ms;
        let predicted_speedup = amdahl_speedup(s, n as f64);

        println!(
            "{:8} | {:10.1} | {:7.2} | {:17.2}",
            n, elapsed_ms, measured_speedup, predicted_speedup
        );

        if n == 8 {
            measured_speedup_at_8 = measured_speedup;
        }
    }

    println!();
    if !correctness_ok {
        eprintln!("Parallel results did not match the sequential baseline. No token issued.");
        std::process::exit(1);
    }

    let student_id = get_student_id();
    println!("STUDENT: {}", student_id);
    println!(
        "SUCCESS TOKEN: AMDAHL-VERIFIED-{}-{:.2}",
        student_id, measured_speedup_at_8
    );
}
