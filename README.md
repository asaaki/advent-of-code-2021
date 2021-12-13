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
# debug = true
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
| `tmp\aoc.win.exe 0 1` | 6.2 ± 0.6 | 5.6 | 9.6 | 1.02 ± 0.10 |
| `tmp\aoc.win.exe 0 2` | 6.1 ± 0.3 | 5.6 | 6.9 | 1.00 |
| `tmp\aoc.win.exe 1 1` | 6.4 ± 0.3 | 5.9 | 7.2 | 1.04 ± 0.07 |
| `tmp\aoc.win.exe 1 2` | 6.2 ± 0.3 | 5.7 | 7.0 | 1.03 ± 0.07 |
| `tmp\aoc.win.exe 2 1` | 6.4 ± 0.4 | 5.9 | 7.5 | 1.06 ± 0.08 |
| `tmp\aoc.win.exe 2 2` | 6.5 ± 0.4 | 5.8 | 7.5 | 1.06 ± 0.09 |
| `tmp\aoc.win.exe 3 1` | 6.3 ± 0.4 | 5.7 | 7.1 | 1.04 ± 0.08 |
| `tmp\aoc.win.exe 3 2` | 6.4 ± 0.2 | 5.7 | 7.1 | 1.05 ± 0.06 |
| `tmp\aoc.win.exe 4 1` | 6.5 ± 0.4 | 5.8 | 7.8 | 1.07 ± 0.08 |
| `tmp\aoc.win.exe 4 2` | 6.6 ± 0.4 | 6.1 | 7.7 | 1.09 ± 0.08 |
| `tmp\aoc.win.exe 5 1` | 10.9 ± 0.4 | 10.3 | 12.1 | 1.80 ± 0.11 |
| `tmp\aoc.win.exe 5 2` | 12.3 ± 0.5 | 11.7 | 13.7 | 2.02 ± 0.13 |
| `tmp\aoc.win.exe 6 1` | 6.1 ± 0.3 | 5.5 | 6.9 | 1.01 ± 0.07 |
| `tmp\aoc.win.exe 6 2` | 6.2 ± 0.3 | 5.7 | 7.3 | 1.02 ± 0.07 |
| `tmp\aoc.win.exe 7 1` | 6.4 ± 0.3 | 5.8 | 7.7 | 1.04 ± 0.07 |
| `tmp\aoc.win.exe 7 2` | 6.5 ± 0.4 | 5.7 | 7.6 | 1.06 ± 0.08 |

### Linux (WSL2; Ubuntu 20.04.3 LTS); toolchain: stable

`make benchmark.wsl`

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `tmp/aoc.linux 0 1` | 0.5 ± 0.0 | 0.5 | 0.7 | 1.00 |
| `tmp/aoc.linux 0 2` | 0.5 ± 0.1 | 0.5 | 0.8 | 1.01 ± 0.14 |
| `tmp/aoc.linux 1 1` | 0.7 ± 0.1 | 0.6 | 1.0 | 1.29 ± 0.15 |
| `tmp/aoc.linux 1 2` | 0.7 ± 0.1 | 0.6 | 0.9 | 1.30 ± 0.15 |
| `tmp/aoc.linux 2 1` | 0.7 ± 0.1 | 0.6 | 1.0 | 1.29 ± 0.15 |
| `tmp/aoc.linux 2 2` | 0.7 ± 0.1 | 0.6 | 1.0 | 1.33 ± 0.18 |
| `tmp/aoc.linux 3 1` | 0.7 ± 0.1 | 0.6 | 1.1 | 1.34 ± 0.17 |
| `tmp/aoc.linux 3 2` | 0.7 ± 0.1 | 0.6 | 1.0 | 1.38 ± 0.16 |
| `tmp/aoc.linux 4 1` | 0.7 ± 0.0 | 0.6 | 1.0 | 1.38 ± 0.15 |
| `tmp/aoc.linux 4 2` | 0.9 ± 0.1 | 0.8 | 1.2 | 1.69 ± 0.19 |
| `tmp/aoc.linux 5 1` | 2.6 ± 0.2 | 2.2 | 3.5 | 4.88 ± 0.58 |
| `tmp/aoc.linux 5 2` | 3.4 ± 0.3 | 3.1 | 4.9 | 6.41 ± 0.79 |
| `tmp/aoc.linux 6 1` | 0.5 ± 0.1 | 0.5 | 0.9 | 1.03 ± 0.14 |
| `tmp/aoc.linux 6 2` | 0.5 ± 0.0 | 0.5 | 0.8 | 1.01 ± 0.12 |
| `tmp/aoc.linux 7 1` | 0.8 ± 0.1 | 0.7 | 1.2 | 1.47 ± 0.19 |
| `tmp/aoc.linux 7 2` | 1.2 ± 0.0 | 1.1 | 1.4 | 2.33 ± 0.21 |

<!-- links -->

[Advent of Code 2021]: https://adventofcode.com/2021
