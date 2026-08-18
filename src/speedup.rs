//! Lab 1: The Speedup Calculator -- library code.
//!
//! UNVALIDATED: no Rust/Cargo toolchain was available in the
//! environment this starter was built in, so nothing here has been
//! compiled. Written carefully by hand -- see the course README for
//! details. Complete the TODOs below.

/// Given: sums all prime numbers below `limit` using trial division.
/// Deliberately unoptimized (that's the point -- it's the CPU-bound
/// sequential baseline you're going to parallelize and benchmark).
pub fn sequential_sum_of_primes(limit: u64) -> u64 {
  let mut sum: u64 = 0;
  for n in 2..limit {
    if is_prime(n) {
      sum += n;
    }
  }
  sum
}

fn is_prime(n: u64) -> bool {
  if n < 2 {
    return false;
  }
  let mut i = 2u64;
  while i * i <= n {
    if n % i == 0 {
      return false;
    }
    i += 1;
  }
  true
}

/// Parallel version: divide [2, limit) into `num_threads` contiguous
/// chunks, one thread per chunk, combine partial sums with a final
/// reduction. See Lab_01_The_Speedup_Calculator.md, Part B,
/// Requirement 1.
pub fn parallel_sum_of_primes(limit: u64, num_threads: usize) -> u64 {
  // TODO: divide [2, limit) into `num_threads` contiguous chunks,
// spawn one std::thread per chunk (each summing primes in its
// chunk via is_prime), join all threads, and sum the partial
// results. std::thread::scope is a clean way to do this without
// needing Arc, since is_prime doesn't need any shared mutable
// state -- only `limit` and each chunk's bounds, which can be
// captured by the closures directly.
raise_not_implemented()
}

fn raise_not_implemented() -> u64 {
  panic!("parallel_sum_of_primes is not implemented yet")
}

/// Estimate the sequential fraction `s` (for Amdahl's Law) by timing
/// a portion of the algorithm that cannot be parallelized. See
/// Lab_01_The_Speedup_Calculator.md, Part B, Requirement 3.
///
/// Suggested approach: time sequential_sum_of_primes at a small fixed
/// limit (e.g. limit / 1000) as a proxy for fixed per-run overhead
/// (setup, the final reduction step), and time the full sequential
/// run at the real `limit`; the ratio of the former to the latter is
/// your estimated `s`. This is a rough estimate, not an exact
/// decomposition -- document your reasoning in Lab1_Theory.pdf.
pub fn measure_sequential_fraction(limit: u64) -> f64 {
  // TODO
raise_not_implemented() as f64
}

pub fn amdahl_speedup(s: f64, n: f64) -> f64 {
  1.0 / (s + (1.0 - s) / n)
}
