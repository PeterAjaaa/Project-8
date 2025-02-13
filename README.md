# Project-8

Project-8 is an interpreter with integrated REPL for Brainf*ck programming language, written in Rust.

## Performance Report

Specs used for benchmark:

CPU: Intel i5-1035G4

GPU: Intel Iris Plus G4

RAM: 8GB Dual Channel (4+4)

Memory: 500GB

OS: ArchLinux (with i3wm)

30 sample, ran without any other application running in the foreground (except the terminal window). Benchmarked using [Hyperfine](https://github.com/sharkdp/hyperfine).

[First iteration](https://github.com/PeterAjaaa/Project-8/tree/2a0bcf4f945527dbdff7fdb6d22afb57edc44c3d):

KEY POINTS:

- Used direct interpretation.

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./project-8 ../../mandelbrot.bf` | 27.215 ± 0.116 | 26.854 | 27.383 | 1.00 |

[Second iteration](https://github.com/PeterAjaaa/Project-8/tree/f6af189707022067e23ef91e372c5da3e68272b6):

KEY POINTS:

- Used custom IR (intermediate representation).

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./project-8 ../../mandelbrot.bf` | 6.308 ± 0.013 | 6.280 | 6.341 | 1.00 |
