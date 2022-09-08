# High Resolution Stopwatch
[![GitHub](https://img.shields.io/github/license/Starz0r/hrstopwatch?style=flat-square)](https://github.com/Starz0r/hrstopwatch) [![crates.io badge](https://badgen.net/crates/v/hrstopwatch?style=flat-square)](https://crates.io/hrstopwatch) [![Docs.rs](https://img.shields.io/docsrs/hrstopwatch/latest?style=flat-square)](https://docs.rs/hrstopwatch/latest) ![rustc requirements](https://img.shields.io/badge/rust-1.49+-brightgreen.svg?logo=rust&style=flat-square)
An extremely accurate clock for taking measurements of the length of time between it starting and stopping. Includes the capability to pause and resume. Inspired by https://github.com/moritzrinow/winwatch. Windows Only.

## Usage

```rust
use hrstopwatch::Stopwatch;

let mut num: u64 = 0;
let mut stopwatch = Stopwatch::start()?;
for i in 0..10000 {
	num += i;
}
stopwatch.stop()?;
println!("seconds to calculate: {}", stopwatch.elapsed_seconds_f64());
```