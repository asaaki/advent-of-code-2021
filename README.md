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
| `tmp\aoc.win.exe 0 1 ` | 7.2 ± 0.4 | 6.3 | 8.2 | 1.00 |
| `tmp\aoc.win.exe 0 2 ` | 7.2 ± 0.5 | 6.1 | 8.6 | 1.00 ± 0.09 |
| `tmp\aoc.win.exe 1 1 ` | 7.5 ± 0.5 | 6.7 | 8.8 | 1.04 ± 0.10 |
| `tmp\aoc.win.exe 1 2 ` | 7.5 ± 0.5 | 6.7 | 8.8 | 1.04 ± 0.09 |
| `tmp\aoc.win.exe 2 1 ` | 7.6 ± 0.5 | 6.8 | 8.9 | 1.07 ± 0.09 |
| `tmp\aoc.win.exe 2 2 ` | 7.5 ± 0.4 | 6.6 | 9.0 | 1.04 ± 0.08 |
| `tmp\aoc.win.exe 3 1 ` | 7.8 ± 0.4 | 7.0 | 8.8 | 1.08 ± 0.08 |
| `tmp\aoc.win.exe 3 2 ` | 8.3 ± 0.5 | 7.5 | 10.0 | 1.15 ± 0.10 |
| `tmp\aoc.win.exe 4 1 ` | 7.6 ± 0.4 | 6.8 | 9.0 | 1.05 ± 0.09 |
| `tmp\aoc.win.exe 4 2 ` | 7.8 ± 0.5 | 6.9 | 9.8 | 1.09 ± 0.10 |

### Linux (WSL2; Ubuntu 20.04.3 LTS)

`make benchmark.wsl`

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `tmp/aoc.linux 0 1 ` | 0.6 ± 0.1 | 0.5 | 0.9 | 1.00 |
| `tmp/aoc.linux 0 2 ` | 0.6 ± 0.1 | 0.5 | 0.9 | 1.01 ± 0.16 |
| `tmp/aoc.linux 1 1 ` | 0.8 ± 0.1 | 0.7 | 1.1 | 1.31 ± 0.21 |
| `tmp/aoc.linux 1 2 ` | 0.8 ± 0.1 | 0.7 | 1.3 | 1.31 ± 0.22 |
| `tmp/aoc.linux 2 1 ` | 0.8 ± 0.1 | 0.7 | 1.0 | 1.29 ± 0.18 |
| `tmp/aoc.linux 2 2 ` | 0.8 ± 0.1 | 0.7 | 1.2 | 1.32 ± 0.20 |
| `tmp/aoc.linux 3 1 ` | 0.8 ± 0.1 | 0.7 | 1.1 | 1.35 ± 0.20 |
| `tmp/aoc.linux 3 2 ` | 1.1 ± 0.1 | 1.0 | 1.4 | 1.80 ± 0.25 |
| `tmp/aoc.linux 4 1 ` | 0.9 ± 0.1 | 0.8 | 1.3 | 1.41 ± 0.20 |
| `tmp/aoc.linux 4 2 ` | 1.1 ± 0.1 | 1.0 | 1.4 | 1.87 ± 0.27 |
<!-- links -->
[Advent of Code 2021]: https://adventofcode.com/2021
