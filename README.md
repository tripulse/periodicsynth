<h1 align=center><code>periodicsynth</code></h2>

<p align=center>
<a href='https://travis-ci.org/tripulse/periodicsynth'><img src='https://travis-ci.org/tripulse/periodicsynth.svg?branch=master'/></a>
<img src='https://docs.rs/periodicsynth/badge.svg'/>
</p>

A periodic waveform generator which calls arbitrary functions
for generating waveforms with an arbitrary samplerate which 
is translated into time-position between (0..1) and provided
to the generator.

Some functions are provided by default (eg. sine, consine,
square, null). To implement a one follow this kind of syntax:

```rust
struct YourData;  // v  you can use your own type.
fn gen(t: f64, dat: YourData) -> f64 { 1.0 }
```

---

Using a default function packed (sin):
```rust
use periodicsynth::{synth, sin};

fn main()
{ let samp = synth(sin, &mut 440f64, 8000); }
```

Using our own and handcrafted function:
```rust
use periodicsynth::synth;

struct YourData;  // you can use any type at here.
fn gen(t: f64, dat: YourData) -> f64 { 1.0 }

fn main()
{ let samp = synth(gen, &mut YourData{}, 8000); }
```

## Motivation
The WebAudio API's [`OscillatorNode`](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode) generates periodic waveforms with different functions. Something
like that doesn't exist so this is created to do just
that or even further to that by allowing end-users to 
define custom functions to generate waveforms.
