# PeriodicSynth
[![Build Status](https://travis-ci.org/nullvideo/periodicsynth.svg?branch=master)](https://travis-ci.org/nullvideo/periodicsynth)

A basic periodic waveform synthesizer for generating the most common types of waveforms that a signal generator is able to generate. Basically, it mimics the capabalities of a signal generator.

---

```bash
periodicsynth = "0.1.2"
```

```rust
/**
 * A basic usage of the library, a triangle wave
 * with a frequency of 440hZ and samplerate of
 * 44.1khZ and bit-depth of 64bits.
 */
use periodicsynth;

fn main() {
    let mut samples = vec![0f64; 44100];
    periodicsynth::synth(&samples, periodicsynth::PereodicFunction::Traingle, 440.0);
}
```


## Motivation
The WebAudio API's [`OscillatorNode`](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode) generates pereodic waveforms on demand with several types of pereodic function types with arbitary frequency and samplerate.
