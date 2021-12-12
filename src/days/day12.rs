use std::{collections::HashMap, ops::AddAssign};

aoc_macros::day_impl_common!();

type Id = String;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Node {
    End,
    Big(Id),
    Small(Id),
}

type Nodes = Vec<Node>;
type CaveSystem = HashMap<Node, Nodes>;
type Visits = HashMap<Id, u8>;

fn compute(input: StrInputRef, first_part: bool) -> usize {
    let (caves, visits) = make_maps(input);
    let start = Node::Small("start".into());
    let mut counts = 0;
    traverse_cave(first_part, &caves, visits, &start, &mut counts);
    counts
}

fn make_maps(input: StrInputRef) -> (CaveSystem, Visits) {
    let mut caves = CaveSystem::new();
    let mut visits = Visits::new();

    for line in input {
        let (l, r) = line.split_once("-").unwrap();
        let (l, r) = (make_node(l), make_node(r));

        let entry = caves.entry(l.clone()).or_insert(Nodes::new());
        entry.push(r.clone());
        let entry = caves.entry(r.clone()).or_insert(Nodes::new());
        entry.push(l.clone());
    }

    for node in caves.keys() {
        if let Node::Small(id) = node {
            visits.insert(id.clone(), 0);
        }
    }

    (caves, visits)
}

fn make_node(s: &str) -> Node {
    use Node::*;

    match s {
        "end" => End,
        _ => {
            if s.chars().all(|c| c.is_ascii_uppercase()) {
                Big(s.to_owned())
            } else {
                Small(s.to_owned())
            }
        }
    }
}

fn traverse_cave(
    first_part: bool,
    caves: &CaveSystem,
    visits: Visits,
    from: &Node,
    counts: &mut usize
) -> () {
    use Node::*;
    let mut visits = visits;

    if let Small(id) = from {
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

    for cave in caves.get(&from).unwrap() {
        if cave == &End {
            counts.add_assign(1);
        } else {
            traverse_cave(first_part, caves, visits.clone(), cave, counts);
        }
    }
}
