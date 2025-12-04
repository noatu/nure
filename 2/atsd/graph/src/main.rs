use std::{
    collections::{BTreeMap, BTreeSet},
    io,
};

#[derive(Default)]
struct Graph {
    vertices: BTreeMap<usize, BTreeMap<usize, u32>>,
}

impl Graph {
    fn rem_edge(&mut self, v1: usize, v2: usize) {
        self.vertices.entry(v1).and_modify(|m| {
            m.remove(&v2);
        });
        self.vertices.entry(v2).and_modify(|m| {
            m.remove(&v1);
        });
    }

    fn add_edge(&mut self, v1: usize, v2: usize, distance: u32) {
        self.vertices.entry(v1).or_default().insert(v2, distance);
        self.vertices.entry(v2).or_default().insert(v1, distance);
    }

    pub fn new() -> Self {
        let mut graph = Self::default();
        println!(
            "Exit - 'q' or 'x'
Print the graph - 'p'
Calculate dijkstra to each vertex - 'd vertex'
Remove an edge - 'vertex1 vertex2'
Add an edge - 'vertex1 vertex2 distance'"
        );

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let args: Vec<&str> = input.split_whitespace().collect();

            match args.len() {
                0 => continue,
                1 => match args[0] {
                    "q" | "x" => break,
                    "p" => graph.print(),
                    _ => println!("Can't parse {input}"),
                },
                2 => {
                    if args[0] == "d" {
                        println!("{:?}", graph.dijkstra(args[1].parse().unwrap()));
                    } else {
                        graph.rem_edge(args[0].parse().unwrap(), args[1].parse().unwrap());
                        println!("Removed {} - {}", args[0], args[1]);
                    }
                }
                3 => {
                    graph.add_edge(
                        args[0].parse().unwrap(),
                        args[1].parse().unwrap(),
                        args[2].parse().unwrap(),
                    );
                    println!("Added {} - {}, {}", args[0], args[1], args[2]);
                }
                _ => println!("Can't parse {input}"),
            }
        }

        graph
    }

    pub fn print(&self) {
        let mut viewed = BTreeSet::new();

        for (v1, vertices) in &self.vertices {
            for (v2, distance) in vertices {
                if viewed.contains(&(v1, v2)) || viewed.contains(&(v2, v1)) {
                    continue;
                }

                println!("{v1} - {v2}, {distance}");
                viewed.insert((v1, v2));
            }
        }
    }

    // find a representative for a given element
    fn find_repr(reprs: &BTreeMap<usize, usize>, element: usize) -> usize {
        if reprs[&element] == element {
            return element;
        }

        Self::find_repr(reprs, reprs[&element])
    }

    pub fn kruskal(&self) -> Self {
        let mut kruskal_graph = Self::default();

        let mut edges = BTreeSet::new();
        let mut reprs = BTreeMap::new();

        for (v1, vertices) in &self.vertices {
            for (v2, distance) in vertices {
                if !edges.contains(&(*v2, *v1, *distance)) {
                    edges.insert((*v1, *v2, *distance));
                }
            }
            reprs.insert(*v1, *v1);
        }

        let mut edges: Vec<_> = edges.into_iter().collect();
        edges.sort_by_key(|k| k.2);

        let mut count = 0;
        for (v1, v2, dist) in edges {
            if count + 1 >= self.vertices.len() {
                break;
            }

            let parent1 = Self::find_repr(&reprs, v1);
            let parent2 = Self::find_repr(&reprs, v2);

            if parent1 != parent2 {
                count += 1;
                kruskal_graph.add_edge(v1, v2, dist);
                reprs.insert(parent2, parent1);
            }
        }

        kruskal_graph
    }

    pub fn dijkstra(&self, start: usize) -> BTreeMap<usize, Option<(usize, u32)>> {
        let mut visited = BTreeMap::new(); // map of vertices and predecessor with distance
        let mut fifo = BTreeMap::new(); // first in first out buffer of vertex and distance

        visited.insert(start, None); // doesn't have a predecessor

        // adding vertices from start
        for (vertex, distance) in &self.vertices[&start] {
            visited.insert(*vertex, Some((start, *distance)));
            fifo.insert(*vertex, *distance);
        }

        while let Some((vertex, distance)) = fifo.pop_first() {
            for (next, next_distance) in &self.vertices[&vertex] {
                let new_distance = distance + next_distance;
                match visited.get(next) {
                    // skip if new distance >= current distance for vertex
                    Some(Some((_, current_distance))) if new_distance >= *current_distance => {}
                    // vertex is a start
                    Some(None) => {}
                    // new distance is shorter or next was not in distances
                    _ => {
                        visited.insert(*next, Some((vertex, new_distance)));
                        fifo.insert(*next, new_distance);
                    }
                }
            }
        }

        visited
    }
}

fn main() {
    let graph = Graph::new();
    graph.print();
    println!();
    graph.kruskal().print();
}
