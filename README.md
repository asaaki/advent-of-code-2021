# [Advent of Code 2021]

My solutions for each task.

## Goals

- Solve the puzzles.
- Solve them as quickly as possible.
- (optional) Refactor if something bothers me too much.
- Yet still: Have fun with the puzzles!
- Enjoy the coding practice and built up the "muscle memory" for Rust.
- Learn more Rust on the way.
- Become more confident.
- Allow rough edges here and there.
- Have fun!

## Non-Goals

- Performance and efficiency.¹
- The most, best, optimal, idiomatic way of writing Rust.²
- Perfectionism.
- Competition.

¹ _Unless the task at hand requires it._
² _Well, a bit of idiomatic code is probably still good._

## Benchmarks

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
debug = true
```

```sh
# simplified version (win10; git bash)
hyperfine \
  --export-markdown tmp\benchmark.md \
  --warmup 10 --runs 50 \
  -L parts 1,2 \
  -L days $(seq -s ',' 0 $(date +"%e")) \
  "tmp\aoc.win.exe {days} {parts}"
```

### Windows 10 (within Git Bash); toolchain: stable-msvc

`make benchmark`

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `tmp\aoc.win.exe 0 1` | 6.7 ± 0.5 | 5.8 | 7.8 | 1.02 ± 0.09 |
| `tmp\aoc.win.exe 0 2` | 6.6 ± 0.3 | 6.0 | 7.3 | 1.00 |
| `tmp\aoc.win.exe 1 1` | 6.8 ± 0.5 | 6.1 | 8.2 | 1.03 ± 0.08 |
| `tmp\aoc.win.exe 1 2` | 6.7 ± 0.3 | 6.2 | 7.5 | 1.01 ± 0.07 |
| `tmp\aoc.win.exe 2 1` | 6.8 ± 0.4 | 6.2 | 8.0 | 1.03 ± 0.07 |
| `tmp\aoc.win.exe 2 2` | 6.7 ± 0.3 | 6.2 | 7.7 | 1.02 ± 0.07 |
| `tmp\aoc.win.exe 3 1` | 6.9 ± 0.4 | 6.2 | 7.9 | 1.04 ± 0.07 |
| `tmp\aoc.win.exe 3 2` | 6.9 ± 0.4 | 6.3 | 8.0 | 1.05 ± 0.08 |
| `tmp\aoc.win.exe 4 1` | 7.0 ± 0.4 | 6.4 | 8.2 | 1.05 ± 0.08 |
| `tmp\aoc.win.exe 4 2` | 7.1 ± 0.4 | 6.1 | 8.2 | 1.08 ± 0.08 |
| `tmp\aoc.win.exe 5 1` | 11.6 ± 0.7 | 10.5 | 13.5 | 1.75 ± 0.14 |
| `tmp\aoc.win.exe 5 2` | 12.9 ± 0.6 | 11.8 | 15.4 | 1.95 ± 0.13 |
| `tmp\aoc.win.exe 6 1` | 6.7 ± 0.5 | 5.9 | 8.6 | 1.02 ± 0.09 |
| `tmp\aoc.win.exe 6 2` | 6.7 ± 0.4 | 6.1 | 7.7 | 1.00 ± 0.07 |

### Linux (WSL2; Ubuntu 20.04.3 LTS); toolchain: stable

`make benchmark.wsl`

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `tmp/aoc.linux 0 1` | 0.5 ± 0.0 | 0.5 | 0.7 | 1.00 |
| `tmp/aoc.linux 0 2` | 0.6 ± 0.1 | 0.5 | 0.9 | 1.19 ± 0.22 |
| `tmp/aoc.linux 1 1` | 0.7 ± 0.1 | 0.6 | 0.8 | 1.33 ± 0.14 |
| `tmp/aoc.linux 1 2` | 0.7 ± 0.0 | 0.6 | 0.8 | 1.33 ± 0.13 |
| `tmp/aoc.linux 2 1` | 0.7 ± 0.1 | 0.6 | 1.0 | 1.34 ± 0.15 |
| `tmp/aoc.linux 2 2` | 0.7 ± 0.1 | 0.6 | 0.9 | 1.33 ± 0.14 |
| `tmp/aoc.linux 3 1` | 0.7 ± 0.1 | 0.6 | 0.9 | 1.37 ± 0.15 |
| `tmp/aoc.linux 3 2` | 0.7 ± 0.1 | 0.7 | 1.0 | 1.37 ± 0.15 |
| `tmp/aoc.linux 4 1` | 0.8 ± 0.1 | 0.7 | 1.1 | 1.51 ± 0.20 |
| `tmp/aoc.linux 4 2` | 0.9 ± 0.1 | 0.8 | 1.1 | 1.72 ± 0.18 |
| `tmp/aoc.linux 5 1` | 2.7 ± 0.3 | 2.3 | 3.8 | 5.14 ± 0.66 |
| `tmp/aoc.linux 5 2` | 3.6 ± 0.4 | 3.1 | 4.9 | 6.77 ± 0.92 |
| `tmp/aoc.linux 6 1` | 0.6 ± 0.0 | 0.5 | 0.6 | 1.03 ± 0.10 |
| `tmp/aoc.linux 6 2` | 0.6 ± 0.0 | 0.5 | 0.7 | 1.04 ± 0.11 |

<!-- links -->

[Advent of Code 2021]: https://adventofcode.com/2021
