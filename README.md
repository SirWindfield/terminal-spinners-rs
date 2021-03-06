# terminal-spinners

> A Rust library for showing terminal loading animations.

<p align="center">
  <img src=".github/misc/demo.dots2.svg" style="height: 200px;">
</p>

## Usage

```rust
use terminal_spinners::{SpinnerBuilder, DOTS};

let handle = SpinnerBuilder::new().spinner(&DOTS).text("Loading unicorns").start();
// Do some other work...
std::thread::sleep(std::time::Duration::from_secs(3));
handle.done();
```

The `examples/` directory contains an example for each available spinner. To see them in action, run `cargo run --example <name>`. Additional examples for showcasing other functionality are:

| Filename     | Desc                                      |
| ------------ | ----------------------------------------- |
| `clear_line` | Showcases `SpinnerHandle::stop_and_clear` |

## Shortcomings

- It's not possible to run multiple spinners at once. This probably needs an API change. Open for help/PR!

#### License

<sup>
Licensed under either of <a href="license-apache">Apache License, Version
2.0</a> or <a href="license-mit">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
