use std::f64::consts::PI;

/// Generate a sinusoid of an arbitrary frequency.
pub fn sin(t: f64, f: &mut f64) -> f64
{ (2.0 * PI * t * *f).sin() }

/// Generate a square-wave of an arbitrary frequency.
pub fn sqr(t: f64, f: &mut f64) -> f64
{ (2.0 * PI * t * *f).sin().signum() }

/// Generate a co-sinusoid of an arbitrary frequency.
pub fn cos(t: f64, f: &mut f64) -> f64
{ (PI * t * *f).sin() }

/// This is just for covinience, generates 0-valued samples.
pub fn null(_: (), _: ()) -> f64 { 0.0 }