// This only works for part 1 full, and part 2 test-only;
// for part 2 challenge somewhere is an "off-by-10" error;
// used https://pastebin.com/AN2FihWe to get the correct answer;
// mine: 2935, theirs/correct: 2925, diff: -10

// alternative to try: https://github.com/samueltardieu/pathfinding

use petgraph::{
    algo::dijkstra,
    graph::{DiGraph, NodeIndex},
};

type Digit = usize;
type WeightMap<'a> = MatrixV<'a, Digit>;
type WeightedEdge = (Digit, Digit, Digit);
type WeightedEdges = Vec<WeightedEdge>;
type MapGraph = DiGraph<Digit, Digit, Digit>;

const TILE_FACTOR: Digit = 5;

aoc_macros::day_impl_common!();

fn compute(input: StrInputRef, first_part: bool) -> usize {
    let factor = input.len();
    let factor = if first_part {
        factor
    } else {
        factor * TILE_FACTOR
    };

    let map = init_map(first_part, input);
    let (start, end) = (0, (map.len() - 1));

    let mut edges = WeightedEdges::new();
    for a in 0..(factor - 1) {
        let b = a + 1;
        for c in 0..factor {
            let w1 = map.get(b, c).unwrap(); // left
            let w2 = map.get(c, b).unwrap(); // bottom
            let e1 = ((a + (factor * c)), (b + (factor * c)), *w1);
            let e2 = ((c + (factor * a)), (c + (factor * b)), *w2);
            edges.push(e1);
            edges.push(e2);
        }
    }

    let g = MapGraph::from_edges(&edges);
    let node_map =
        dijkstra(&g, start.into(), Some(end.into()), |e| *e.weight());
    *node_map.get(&NodeIndex::new(end)).unwrap()
}

fn init_map(first_part: bool, input: StrInputRef) -> WeightMap {
    let len = input.len();
    let tiles = if first_part { 1 } else { TILE_FACTOR };

    let chunk_len = tiles * len;
    let mut map = WeightMap::new(chunk_len);
    map.fill_iter(std::iter::repeat(0).take(chunk_len * chunk_len));

    for (y, line) in input.iter().enumerate() {
        for ty in 0..tiles {
            for (x, c) in line.bytes().enumerate() {
                let lv = (c - b'0') as usize;
                for tx in 0..tiles {
                    let v = (lv + tx + ty - 1) % 9 + 1;
                    map.insert(x + (tx * len), y + (ty * len), v);
                }
            }
        }
    }

    map
}
