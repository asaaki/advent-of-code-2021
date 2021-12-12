use std::{collections::HashMap, ops::AddAssign};

aoc_macros::day_impl_common!();

type Id<'a> = &'a str;
type Nodes<'a> = Vec<Id<'a>>;
type CaveSystem<'a> = HashMap<Id<'a>, Nodes<'a>>;
type Visits<'a> = HashMap<Id<'a>, u8>;

fn compute(input: StrInputRef, first_part: bool) -> usize {
    let (caves, visits) = make_maps(input);
    let mut counts = 0;
    traverse_cave(first_part, &caves, visits, "start", &mut counts);
    counts
}

fn make_maps(input: StrInputRef) -> (CaveSystem, Visits) {
    let mut caves = CaveSystem::new();
    let mut visits = Visits::new();

    for line in input {
        let (l, r) = line.split_once("-").unwrap();

        let entry = caves.entry(&l).or_insert(Nodes::new());
        entry.push(&r);
        let entry = caves.entry(&r).or_insert(Nodes::new());
        entry.push(&l);
    }

    for node in caves.keys() {
        if node.chars().all(|c| c.is_ascii_lowercase()) {
            visits.insert(node, 0);
        }
    }

    (caves, visits)
}

fn traverse_cave(
    first_part: bool,
    caves: &CaveSystem,
    visits: Visits,
    id: &str,
    counts: &mut usize,
) -> () {
    let mut visits = visits;

    if id.chars().all(|c| c.is_ascii_lowercase()) {
        let &c = visits.get(id).unwrap();
        if first_part {
            if c > 0 {
                return;
            } else {
                visits.get_mut(id).unwrap().add_assign(1);
            }
        } else {
            if id == "start" && visits.iter().any(|(_, &v)| v > 0) {
                return;
            }

            if c > 0 {
                if visits.iter().all(|(_, &v)| v < 2) {
                    visits.get_mut(id).unwrap().add_assign(1);
                } else {
                    return;
                }
            } else {
                visits.get_mut(id).unwrap().add_assign(1);
            }
        }
    }

    for &cave in caves.get(id).unwrap() {
        if cave == "end" {
            counts.add_assign(1);
        } else {
            traverse_cave(first_part, caves, visits.clone(), cave, counts);
        }
    }
}
