use core::fmt;
use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl fmt::Display for JunctionBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JB({},{},{})", self.x, self.y, self.z)
    }
}

impl JunctionBox {
    fn distance(&self, other: &JunctionBox) -> f64 {
        let mut sum = (self.x - other.x).pow(2);
        sum += (self.y - other.y).pow(2);
        sum += (self.z - other.z).pow(2);
        return (sum as f64).sqrt();
    }
}

struct UnionFind<'a> {
    parent: HashMap<&'a JunctionBox, &'a JunctionBox>,
}

impl<'a> UnionFind<'a> {
    fn new(junction_boxes: &'a Vec<JunctionBox>) -> Self {
        let mut s = Self {
            parent: HashMap::new(),
        };

        for jbox in junction_boxes {
            s.parent.insert(jbox, jbox);
        }
        return s;
    }

    fn find(&self, jbox: &'a JunctionBox) -> &'a JunctionBox {
        assert!(self.parent.contains_key(jbox));

        if self.parent[jbox] != jbox {
            return self.find(self.parent[jbox]);
        }
        return jbox;
    }

    fn union(&mut self, a: &'a JunctionBox, b: &'a JunctionBox) -> bool {
        assert!(self.parent.contains_key(a));
        assert!(self.parent.contains_key(b));

        if self.find(a) == self.find(b) {
            // println!(
            //     "Skip merge: jbox {} with jbox {} have the same parent",
            //     a, b
            // );
            return false;
        }
        self.parent.insert(self.find(a), self.find(b));
        return true;
    }

    fn nodes(&self) -> impl Iterator<Item = &&JunctionBox> {
        self.parent.keys()
    }

    fn circuits(&self) -> HashMap<&JunctionBox, Vec<&JunctionBox>> {
        let mut circuits: HashMap<&JunctionBox, Vec<&JunctionBox>> = HashMap::new();
        for node in self.nodes() {
            let parent = self.find(node);
            if !circuits.contains_key(parent) {
                circuits.insert(parent, Vec::new());
            }
            let circuit = circuits.get_mut(parent).unwrap();
            circuit.push(node);
        }
        return circuits;
    }
}

fn main() {
    // let input = fs::read_to_string("inputs/day_08-example.txt").expect("Coud not read input file");
    // let total_pairs_to_connect = 10;
    let input = fs::read_to_string("inputs/day_08.txt").expect("Coud not read input file");
    let total_pairs_to_connect = 1000;

    // Parse the junction boxes from input
    let mut jboxes: Vec<JunctionBox> = Vec::new();
    for line in input.lines() {
        let (xstr, line_rest) = line.split_once(",").unwrap();
        let (ystr, zstr) = line_rest.split_once(",").unwrap();
        let jbox = JunctionBox {
            x: xstr.parse().expect("Couldn't parse coord x"),
            y: ystr.parse().expect("Couldn't parse coord y"),
            z: zstr.parse().expect("Couldn't parse coord z"),
        };

        jboxes.push(jbox);
    }
    println!("There are a total of {} junction boxes", jboxes.len());

    // Calculate distances between each junction box
    let mut pair_dists: Vec<(&JunctionBox, &JunctionBox, f64)> = Vec::new();
    for a in &jboxes {
        for b in &jboxes {
            if a == b {
                continue;
            }
            pair_dists.push((a, b, a.distance(b)));
        }
    }
    pair_dists.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    pair_dists = pair_dists.chunks_exact(2).map(|chunk| chunk[0]).collect();
    println!("There are {} pairs", pair_dists.len());

    println!("=============== PART 1 ===============");
    // Create UnionFind and merge based on connections
    let mut union_find = UnionFind::new(&jboxes);
    for (i, pair) in pair_dists.iter().enumerate() {
        // limit to X shortest connections
        if i == total_pairs_to_connect {
            break;
        }

        let (jbox_a, jbox_b, dist) = pair;
        union_find.union(&jbox_a, &jbox_b);
        println!(
            "Joined together {} and {} with dist {}",
            jbox_a, jbox_b, dist
        );
    }

    // convert from UnionFind to HashMap
    let circuits = union_find.circuits();
    for (parent, nodes) in &circuits {
        println!(
            "circuit size {} parent={}: {:?}",
            nodes.len(),
            parent,
            nodes
        );
    }

    // display result
    println!("There are {} unique circuits", circuits.len());
    let mut circuits_sizes: Vec<usize> = circuits.iter().map(|circuit| circuit.1.len()).collect();
    circuits_sizes.sort();
    circuits_sizes.reverse();
    // multiplication of three largest circuits' sizes
    let result: usize = circuits_sizes
        .first_chunk::<3>()
        .unwrap()
        .iter()
        .fold(1, |acc, c| acc * c);
    println!("Result: {}", result);

    println!("=============== PART 2 ===============");
    // Create UnionFind and merge based on connections
    let mut union_find = UnionFind::new(&jboxes);
    let mut total_circuits = jboxes.len();
    for (jbox_a, jbox_b, _dist) in &pair_dists {
        let merged = union_find.union(&jbox_a, &jbox_b);
        println!(
            "Joined together {} and {} with dist {}",
            jbox_a, jbox_b, _dist
        );

        if merged {
            total_circuits -= 1
        }

        if total_circuits == 1 {
            println!("Result for part 2: {}", jbox_a.x * jbox_b.x);
            return;
        }
        println!("circuit_size: {}", total_circuits);
    }
}
