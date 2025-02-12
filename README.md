# Project-8

Project-8 is a compiler for Brainf*ck programming language with integrated REPL, written in Rust.

## Performance Report

30 sample, ran without any other application running in the foreground (except the terminal window). Benchmarked using [Hyperfine](https://github.com/sharkdp/hyperfine).

[First iteration](https://github.com/PeterAjaaa/Project-8/tree/2a0bcf4f945527dbdff7fdb6d22afb57edc44c3d):

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./project-8 ../../mandelbrot.bf` | 27.215 ± 0.116 | 26.854 | 27.383 | 1.00 |

[Second iteration](https://github.com/PeterAjaaa/Project-8/tree/f6af189707022067e23ef91e372c5da3e68272b6):

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./project-8 ../../mandelbrot.bf` | 6.308 ± 0.013 | 6.280 | 6.341 | 1.00 |
