use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn run(input: &str) -> u32 {
    let mut graph = FxHashMap::default();
    for line in input.lines() {
        let mut split = line.split([':', ' ']).filter(|l| !l.is_empty());
        let vertex = split.next().unwrap();
        let edges = split.collect_vec();
        for edge in edges {
            graph
                .entry(vertex)
                .or_insert_with(Vec::new)
                .push(edge.to_string());
            graph
                .entry(edge)
                .or_insert_with(Vec::new)
                .push(vertex.to_string());
        }
    }

    let (s1, s2) = minimum_cut(&graph);
    s1 * s2
}

// Stoer-Wagner algorithm
// As output we want number of edges in each set
fn minimum_cut(graph: &FxHashMap<&str, Vec<String>>) -> (u32, u32) {
    let mut graph = graph.clone();
    //Pick the first one to start with
    let vertex = *graph.keys().next().unwrap();
    let mut merged_count = 1;
    let original_graph_size = graph.len();
    while graph[vertex].len() > 3 {
        let edges = graph.get_mut(vertex).unwrap();
        let frequencies = edges.iter().counts();
        let mut max = 0;
        let mut max_edge = String::new();
        for (edge, count) in frequencies {
            if count > max {
                max = count;
                max_edge = edge.clone();
            }
        }

        //Remove the connections between our merged vertices
        edges.retain(|e| e != max_edge.as_str());
        //Add every edge that was connected to max_edge
        let mut to_add = graph
            .get(max_edge.as_str())
            .unwrap()
            .into_iter()
            .filter(|e| *e != vertex)
            .cloned()
            .collect_vec();

        let edges = graph.get_mut(vertex).unwrap();
        edges.append(&mut to_add);

        //Take every edge currently connected to the one we're merging
        //And change it to instead lead to the other vertex
        for edge in graph.get(max_edge.as_str()).unwrap().clone() {
            for e in graph.get_mut(edge.as_str()).unwrap() {
                if *e == max_edge {
                    *e = vertex.to_owned();
                }
            }
        }

        //Delete max_edge
        graph.remove(max_edge.as_str());
        merged_count += 1;
    }
    (merged_count, original_graph_size as u32 - merged_count)
}

#[test]
fn example() {
    let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    assert_eq!(run(input), 54);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("inputs/2023/day25.txt").unwrap();
    assert_eq!(run(&input), 546804);
}

#[divan::bench]
fn bench(bencher: divan::Bencher) {
    let input = std::fs::read_to_string("inputs/2023/day25.txt").unwrap();
    bencher.bench(|| run(divan::black_box(&input)));
}
