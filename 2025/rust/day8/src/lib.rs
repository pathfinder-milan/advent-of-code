pub mod parser;

#[derive(Debug, Clone, Copy)]
pub struct JunctionBox {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl JunctionBox {
    pub fn euclidean_distance(&self, other: &JunctionBox) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Debug)]
pub struct Edge {
    pub distance: f64,
    pub box1: usize,
    pub box2: usize,
}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        let (root_x, root_y) = if self.rank[root_x] < self.rank[root_y] {
            (root_y, root_x)
        } else {
            (root_x, root_y)
        };

        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];

        if self.rank[root_x] == self.rank[root_y] {
            self.rank[root_x] += 1;
        }

        true
    }

    pub fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let mut circuits = std::collections::HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            circuits.insert(root, self.size[root]);
        }
        circuits.values().copied().collect()
    }

    pub fn get_num_circuits(&mut self) -> usize {
        let mut roots = std::collections::HashSet::new();
        for i in 0..self.parent.len() {
            roots.insert(self.find(i));
        }
        roots.len()
    }
}

pub fn get_sorted_edges(boxes: &[JunctionBox]) -> Vec<Edge> {
    let n = boxes.len();
    let mut edges = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let distance = boxes[i].euclidean_distance(&boxes[j]);
            edges.push(Edge {
                distance,
                box1: i,
                box2: j,
            });
        }
    }

    edges.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    edges
}

pub fn multiply_three_largest_circuit_sizes(
    boxes: &[JunctionBox],
    num_connections: usize,
) -> usize {
    let mut uf = UnionFind::new(boxes.len());
    let edges = get_sorted_edges(boxes);

    for i in 0..num_connections.min(edges.len()) {
        uf.union(edges[i].box1, edges[i].box2);
    }

    let mut circuit_sizes = uf.get_circuit_sizes();
    circuit_sizes.sort_by(|a, b| b.cmp(a));

    circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]
}

pub fn multiply_last_two_junction_box_x_coordinates(boxes: &[JunctionBox]) -> i32 {
    let mut union_find = UnionFind::new(boxes.len());
    let edges = get_sorted_edges(boxes);

    let mut last_connection = None;
    for edge in edges {
        if union_find.union(edge.box1, edge.box2) {
            last_connection = Some((edge.box1, edge.box2));
            if union_find.get_num_circuits() == 1 {
                break;
            }
        }
    }

    if let Some((i, j)) = last_connection {
        boxes[i].x * boxes[j].x
    } else {
        0
    }
}
