use std::collections::{HashSet, VecDeque};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
struct Vertex(u32);

impl Vertex {
    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| Vertex::from(e.1.clone()))
            .collect()
    }
}

impl From<u32> for Vertex {
    fn from(item: u32) -> Self {
        Vertex(item)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
struct Edge(u32, u32);

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
        Edge(item.0, item.1)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(vertices: Vec<u32>, edges: Vec<(u32, u32)>) -> Graph {
        Graph {
            vertices: vertices.into_iter().map(|v| Vertex::from(v)).collect(),
            edges: edges.into_iter().map(|e| Edge::from((e.0, e.1))).collect(),
        }
    }

    fn traverse(&self) {
        let mut visited: HashSet<Vertex> = HashSet::new();

        for vertex in &self.vertices {
            visited.insert(*vertex);

            for neighbor in vertex.neighbors(self).into_iter() {
                if visited.insert(neighbor) {
                    println!(
                        "Tree edge current vertex {:?} to neighbor {:?}",
                        vertex, neighbor
                    );
                } else {
                    println!(
                        "-- Back edge current vertex {:?} to neighbor {:?}",
                        vertex, neighbor
                    );
                }
            }
        }
    }
}

fn main() {
    let vertices: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let edges: Vec<(u32, u32)> = vec![
        (1, 2),
        (2, 3),
        (1, 5),
        (2, 5),
        (3, 6),
        (5, 6),
        (6, 9),
        (4, 7),
        (4, 8),
        (7, 8),
    ];

    let graph = Graph::new(vertices, edges);
    graph.traverse();
}
