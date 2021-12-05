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

```sh
# simplified version (win10; git bash)
hyperfine \
  --export-markdown tmp\benchmark.md \
  --warmup 10 --runs 50 \
  -L parts 1,2 \
  -L days $(seq -s ',' 0 $(date +"%e")) \
  "tmp\aoc.win.exe {days} {parts}"
```

### Windows 10 (within Git Bash)

`make benchmark`

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `tmp\aoc.win.exe 0 1` | 8.2 ± 0.5 | 7.4 | 9.5 | 1.01 ± 0.08 |
| `tmp\aoc.win.exe 0 2` | 8.1 ± 0.5 | 7.3 | 9.8 | 1.00 |
| `tmp\aoc.win.exe 1 1` | 8.5 ± 0.4 | 7.8 | 9.8 | 1.04 ± 0.08 |
| `tmp\aoc.win.exe 1 2` | 8.5 ± 0.4 | 7.7 | 9.1 | 1.04 ± 0.08 |
| `tmp\aoc.win.exe 2 1` | 8.5 ± 0.4 | 7.8 | 9.4 | 1.04 ± 0.08 |
| `tmp\aoc.win.exe 2 2` | 8.4 ± 0.4 | 7.6 | 9.4 | 1.03 ± 0.08 |
| `tmp\aoc.win.exe 3 1` | 8.4 ± 0.3 | 7.8 | 9.2 | 1.03 ± 0.07 |
| `tmp\aoc.win.exe 3 2` | 8.6 ± 0.5 | 7.8 | 10.2 | 1.06 ± 0.09 |
| `tmp\aoc.win.exe 4 1` | 8.6 ± 0.4 | 7.8 | 9.8 | 1.05 ± 0.08 |
| `tmp\aoc.win.exe 4 2` | 8.8 ± 0.4 | 7.8 | 9.8 | 1.08 ± 0.08 |
| `tmp\aoc.win.exe 5 1` | 13.8 ± 0.6 | 12.7 | 15.0 | 1.70 ± 0.12 |
| `tmp\aoc.win.exe 5 2` | 15.4 ± 0.6 | 14.1 | 16.4 | 1.89 ± 0.13 |

### Linux (WSL2; Ubuntu 20.04.3 LTS)

`make benchmark.wsl`

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `tmp/aoc.linux 0 1` | 0.6 ± 0.0 | 0.5 | 0.7 | 1.00 |
| `tmp/aoc.linux 0 2` | 0.7 ± 0.2 | 0.5 | 1.3 | 1.15 ± 0.29 |
| `tmp/aoc.linux 1 1` | 0.8 ± 0.1 | 0.7 | 1.0 | 1.35 ± 0.16 |
| `tmp/aoc.linux 1 2` | 0.8 ± 0.1 | 0.7 | 1.0 | 1.34 ± 0.15 |
| `tmp/aoc.linux 2 1` | 0.8 ± 0.1 | 0.7 | 1.1 | 1.41 ± 0.18 |
| `tmp/aoc.linux 2 2` | 0.8 ± 0.1 | 0.7 | 0.9 | 1.35 ± 0.12 |
| `tmp/aoc.linux 3 1` | 0.9 ± 0.1 | 0.7 | 1.3 | 1.59 ± 0.27 |
| `tmp/aoc.linux 3 2` | 0.8 ± 0.1 | 0.7 | 1.0 | 1.40 ± 0.14 |
| `tmp/aoc.linux 4 1` | 0.8 ± 0.1 | 0.8 | 1.2 | 1.46 ± 0.15 |
| `tmp/aoc.linux 4 2` | 1.0 ± 0.1 | 0.9 | 1.2 | 1.74 ± 0.14 |
| `tmp/aoc.linux 5 1` | 3.1 ± 0.3 | 2.5 | 3.9 | 5.39 ± 0.64 |
| `tmp/aoc.linux 5 2` | 4.1 ± 0.3 | 3.6 | 5.0 | 7.03 ± 0.71 |

<!-- links -->

[Advent of Code 2021]: https://adventofcode.com/2021
