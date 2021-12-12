use std::collections::HashMap;

type Id<'a> = &'a str;
type Nodes<'a> = Vec<Id<'a>>;
type CaveSystem<'a> = HashMap<Id<'a>, Nodes<'a>>;
type CavePath<'a> = Vec<Id<'a>>;
type CavePathRef<'a> = &'a [Id<'a>];

const START: &str = "start";
const END: &str = "end";

aoc_macros::day_impl_common!();

fn compute(input: StrInputRef, first_part: bool) -> usize {
    traverse_cave(!first_part, &make_caves(input), START, vec![START])
}

#[inline]
fn make_caves(input: StrInputRef) -> CaveSystem {
    let mut caves = CaveSystem::new();
    for line in input {
        let (l, r) = line.split_once("-").unwrap();
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
    id: Id,
    path: CavePath,
) -> usize {
    caves.get(id).unwrap().iter().fold(0, |count, &node| {
        if node == START {
            return count;
        }
        if node == END {
            return count + 1;
        }
        if is_lower(node) && path.contains(&node) {
            if twice {
                return count
                    + traverse_cave(
                        false,
                        caves,
                        node,
                        new_path_with(node, &path[..]),
                    );
            }
            return count;
        }

        count + traverse_cave(twice, caves, node, new_path_with(node, &path[..]))
    })
}

#[inline]
fn is_lower(id: Id) -> bool {
    id.chars().all(|c| c.is_ascii_lowercase())
}

#[inline]
fn new_path_with<'a>(node: Id<'a>, path: CavePathRef<'a>) -> CavePath<'a> {
    let mut new_path = path.to_vec();
    new_path.push(node);
    new_path
}
