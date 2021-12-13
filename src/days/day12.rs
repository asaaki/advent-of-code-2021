// even though it's said that default HS also uses aHash,
// using AHashMap directly is still faster
use ahash::AHashMap as Map;

// 64 bit Node data
// use ahash::AHasher as NodeDataHasher;
// use std::hash::{Hash, Hasher};
// type NodeData = u64;

// use a 32bit hash for the node data, so the enum becomes smaller
// performance difference to 64 bits is in the microseconds though
use hash32::{FnvHasher as NodeDataHasher, Hash, Hasher};
type NodeData = u32;

type Nodes = Vec<Node>;
type CaveSystem<'a> = Map<Node, Nodes>;
type CavePath<'a> = Vec<&'a Node>;
type CavePathRef<'a> = &'a [&'a Node];

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Node {
    Start,
    End,
    Small(NodeData),
    Big(NodeData),
}

impl Node {
    fn new(s: &str) -> Self {
        if s == "start" {
            Node::Start
        } else if s == "end" {
            Node::End
        } else if s.chars().next().unwrap().is_lowercase() {
            let mut hs = NodeDataHasher::default();
            s.hash(&mut hs);
            Node::Small(hs.finish())
        } else {
            let mut hs = NodeDataHasher::default();
            s.hash(&mut hs);
            Node::Big(hs.finish())
        }
    }
}

aoc_macros::day_impl_common!();

fn compute(input: StrInputRef, first_part: bool) -> usize {
    traverse_cave(
        !first_part,
        &make_caves(input),
        &Node::Start,
        vec![&Node::Start],
    )
}

#[inline]
fn make_caves(input: StrInputRef) -> CaveSystem {
    let mut caves = CaveSystem::new();
    for line in input {
        let (l, r) = line.split_once("-").unwrap();
        let (l, r) = (Node::new(l), Node::new(r));
        let entry = caves.entry(l).or_insert_with(Nodes::new);
        entry.push(r);
        let entry = caves.entry(r).or_insert_with(Nodes::new);
        entry.push(l);
    }
    caves
}

fn traverse_cave(
    twice: bool,
    caves: &CaveSystem,
    current: &Node,
    path: CavePath,
) -> usize {
    caves.get(current).unwrap().iter().fold(0, |count, &node| {
        match node {
            Node::Start => return count,
            Node::End => return count + 1,
            Node::Small(_) => {
                if path.contains(&&node) {
                    if twice {
                        return count
                            + traverse_cave(
                                false,
                                caves,
                                &node,
                                new_path_with(&node, &path[..]),
                            );
                    }
                    return count;
                }
            }
            _ => (),
        }

        count
            + traverse_cave(
                twice,
                caves,
                &node,
                new_path_with(&node, &path[..]),
            )
    })
}

#[inline]
fn new_path_with<'a>(node: &'a Node, path: CavePathRef<'a>) -> CavePath<'a> {
    let mut new_path = path.to_vec();
    new_path.push(node);
    new_path
}
