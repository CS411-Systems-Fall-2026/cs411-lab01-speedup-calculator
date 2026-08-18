//! Lab 1: The Speedup Calculator -- plot binary.
//! UNVALIDATED -- see the note at the top of src/speedup.rs.
//!
//! Run: cargo run --bin plot [--limit N]
//! Produces speedup_plot.png: measured vs. Amdahl-predicted speedup,
//! y-axis = speedup, x-axis = thread count.

use plotters::prelude::*;
use speedup::{amdahl_speedup, measure_sequential_fraction, parallel_sum_of_primes, sequential_sum_of_primes};
use std::env;
use std::time::Instant;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let limit = parse_limit_arg();
  let s = measure_sequential_fraction(limit);

let start = Instant::now();
  sequential_sum_of_primes(limit);
  let sequential_time = start.elapsed().as_secs_f64() * 1000.0;

let thread_counts = [1usize, 2, 4, 8];
  let mut measured: Vec<(f64, f64)> = Vec::new();
  let mut predicted: Vec<(f64, f64)> = Vec::new();

for &n in &thread_counts {
  let t0 = Instant::now();
  parallel_sum_of_primes(limit, n);
  let elapsed_ms = t0.elapsed().as_secs_f64() * 1000.0;
  let measured_speedup = sequential_time / elapsed_ms;
  measured.push((n as f64, measured_speedup));
  predicted.push((n as f64, amdahl_speedup(s, n as f64)));
}

let max_speedup = measured
  .iter()
  .chain(predicted.iter())
  .map(|(_, y)| *y)
  .fold(1.0_f64, f64::max);

let root = BitMapBackend::new("speedup_plot.png", (800, 600)).into_drawing_area();
  root.fill(&WHITE)?;

let mut chart = ChartBuilder::on(&root)
  .caption("Lab 1: Measured vs. Amdahl-Predicted Speedup", ("sans-serif", 24))
  .margin(20)
  .x_label_area_size(40)
  .y_label_area_size(40)
  .build_cartesian_2d(0.0..9.0, 0.0..(max_speedup * 1.1))?;

chart.configure_mesh().x_desc("Thread Count").y_desc("Speedup").draw()?;

chart
  .draw_series(LineSeries::new(measured.clone(), &RED))?
  .label("Measured")
  .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

chart
  .draw_series(LineSeries::new(predicted.clone(), &BLUE))?
  .label("Amdahl Predicted")
  .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

chart
  .configure_series_labels()
  .background_style(&WHITE.mix(0.8))
  .border_style(&BLACK)
  .draw()?;

root.present()?;
  println!("Wrote speedup_plot.png");
  Ok(())
}
