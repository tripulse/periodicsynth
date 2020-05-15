//! This crate is a minimalist crate to generate signal
//! with arbitrary functions to a limited time-resolution.
//! 
//! Some functions are given by default (eg. square,
//! sine, cosine, null).
//! 
//! # Example
//! ```
//! use periodicsynth::{sin, synth};
//! 
//! fn main()
//! { let samp = synth(sin, 440.0f, 8000); }
//! ```

mod common;
pub use common::*;

/// Synthesize a signal using a defined number of
/// samples and a custom function.
/// 
/// # Arguments
/// - `func` — function to use for callback to
/// generate amplitude values.
/// 
/// - `data` — function specific mutable data/
/// data-structure to pass-in.
///
/// - `n` — number of samples to generate (controls
/// the time-resolution).
pub fn synth<'a, U, F: Fn(f64, &mut U) -> f64>
(func: F, data: &mut U, n: usize) -> Vec<f64> {
    let mut samples = Vec::<f64>::with_capacity(n);
    
    let step_factor = 1.0/n as f64;
    let mut time_pos = 0f64;

    for _ in 0..n {
        samples.push(func(time_pos, data));
        time_pos+= step_factor;
    } samples
}