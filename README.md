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
| `tmp\aoc.win.exe 0 1` | 6.7 ± 0.3 | 6.2 | 7.5 | 1.01 ± 0.08 |
| `tmp\aoc.win.exe 0 2` | 6.6 ± 0.4 | 5.9 | 7.9 | 1.00 |
| `tmp\aoc.win.exe 1 1` | 6.9 ± 0.4 | 6.2 | 8.0 | 1.05 ± 0.09 |
| `tmp\aoc.win.exe 1 2` | 6.9 ± 0.3 | 6.3 | 7.7 | 1.04 ± 0.08 |
| `tmp\aoc.win.exe 2 1` | 6.9 ± 0.4 | 6.1 | 7.9 | 1.04 ± 0.09 |
| `tmp\aoc.win.exe 2 2` | 7.0 ± 0.4 | 6.2 | 7.9 | 1.06 ± 0.09 |
| `tmp\aoc.win.exe 3 1` | 7.0 ± 0.4 | 6.2 | 8.8 | 1.06 ± 0.09 |
| `tmp\aoc.win.exe 3 2` | 7.1 ± 0.4 | 6.3 | 8.0 | 1.07 ± 0.09 |
| `tmp\aoc.win.exe 4 1` | 7.0 ± 0.4 | 6.4 | 8.3 | 1.07 ± 0.09 |
| `tmp\aoc.win.exe 4 2` | 7.2 ± 0.3 | 6.4 | 8.0 | 1.08 ± 0.08 |
| `tmp\aoc.win.exe 5 1` | 11.5 ± 0.5 | 10.8 | 13.0 | 1.74 ± 0.13 |
| `tmp\aoc.win.exe 5 2` | 12.8 ± 0.6 | 11.8 | 15.1 | 1.94 ± 0.15 |

### Linux (WSL2; Ubuntu 20.04.3 LTS); toolchain: stable

`make benchmark.wsl`

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `tmp/aoc.linux 0 1` | 0.6 ± 0.0 | 0.5 | 0.7 | 1.00 |
| `tmp/aoc.linux 0 2` | 0.6 ± 0.1 | 0.5 | 1.0 | 1.06 ± 0.20 |
| `tmp/aoc.linux 1 1` | 0.8 ± 0.1 | 0.6 | 1.1 | 1.38 ± 0.23 |
| `tmp/aoc.linux 1 2` | 0.7 ± 0.1 | 0.6 | 1.0 | 1.34 ± 0.17 |
| `tmp/aoc.linux 2 1` | 0.7 ± 0.1 | 0.7 | 1.0 | 1.30 ± 0.16 |
| `tmp/aoc.linux 2 2` | 0.8 ± 0.1 | 0.7 | 1.0 | 1.35 ± 0.17 |
| `tmp/aoc.linux 3 1` | 0.7 ± 0.1 | 0.7 | 1.0 | 1.31 ± 0.17 |
| `tmp/aoc.linux 3 2` | 0.8 ± 0.1 | 0.7 | 1.1 | 1.40 ± 0.20 |
| `tmp/aoc.linux 4 1` | 0.8 ± 0.1 | 0.7 | 1.2 | 1.42 ± 0.22 |
| `tmp/aoc.linux 4 2` | 0.9 ± 0.1 | 0.8 | 1.3 | 1.68 ± 0.20 |
| `tmp/aoc.linux 5 1` | 2.9 ± 0.3 | 2.5 | 3.6 | 5.29 ± 0.66 |
| `tmp/aoc.linux 5 2` | 3.8 ± 0.4 | 3.3 | 5.0 | 6.86 ± 0.89 |

<!-- links -->

[Advent of Code 2021]: https://adventofcode.com/2021
