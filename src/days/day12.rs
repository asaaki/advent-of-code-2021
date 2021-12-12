use std::collections::HashMap;

aoc_macros::day_impl_common!();

type Id<'a> = &'a str;
type Nodes<'a> = Vec<Id<'a>>;
type CaveSystem<'a> = HashMap<Id<'a>, Nodes<'a>>;
type CavePath<'a> = Vec<Id<'a>>;

fn compute(input: StrInputRef, first_part: bool) -> usize {
    traverse_cave(!first_part, &make_caves(input), "start", vec!["start"])
}

#[inline]
fn make_caves(input: StrInputRef) -> CaveSystem {
    let mut caves = CaveSystem::new();
    for line in input {
        let (l, r) = line.split_once("-").unwrap();
        let entry = caves.entry(&l).or_insert(Nodes::new());
        entry.push(&r);
        let entry = caves.entry(&r).or_insert(Nodes::new());
        entry.push(&l);
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
        if node == "start" {
            return count;
        }
        if node == "end" {
            return count + 1;
        }
        if is_lower(node) && path.contains(&node) {
            if twice {
                return count
                    + traverse_cave(
                        false,
                        caves,
                        node,
                        new_path_with(&node, &path),
                    );
            }
            return count;
        }

        count + traverse_cave(twice, caves, node, new_path_with(&node, &path))
    })
}

#[inline]
fn is_lower(id: Id) -> bool {
    id.chars().all(|c| c.is_ascii_lowercase())
}

#[inline]
fn new_path_with<'a>(node: &'a Id, path: &'a CavePath) -> CavePath<'a> {
    let mut new_path = path.clone();
    new_path.push(node);
    new_path
}
