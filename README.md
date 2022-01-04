# README

For a complete (installation) guide follow the instructions [here](https://docs.zama.ai/concrete/lib/installation.html).

To build and run on debian linux the following commands can be used:

```
// Install rust
curl  --tlsv1.2 -sSf https://sh.rustup.rs | sh

// Install FFTW
sudo apt-get update && sudo apt-get install -y libfftw3-dev

// Build and run
RUSTFLAGS="-C target-cpu=native" cargo run --release
```