use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

// Struktur untuk menyimpan posisi pada grid
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

// Struktur untuk node dalam algoritma A*
#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    point: Point,
    g_cost: usize, // Biaya dari start ke node ini
    h_cost: usize, // Estimasi biaya ke tujuan (heuristik)
    parent: Option<Point>,
}

impl Node {
    fn f_cost(&self) -> usize {
        self.g_cost + self.h_cost
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost().cmp(&self.f_cost()) // Dibalik agar BinaryHeap menjadi min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn astar(grid: &[Vec<bool>], start: Point, goal: Point) -> Option<Vec<Point>> {
    let mut open_set = BinaryHeap::new();
    let mut closed_set: HashMap<Point, Node> = HashMap::new();

    open_set.push(Node {
        point: start,
        g_cost: 0,
        h_cost: manhattan_distance(start, goal),
        parent: None,
    });

    while let Some(current) = open_set.pop() {
        if current.point == goal {
            // Rekonstruksi jalur dari tujuan ke start
            let mut path = vec![current.point];
            let mut parent = current.parent;
            while let Some(p) = parent {
                path.push(p);
                parent = closed_set.get(&p).unwrap().parent;
            }
            path.reverse();
            return Some(path);
        }

        closed_set.insert(current.point, current.clone());

        for neighbor in get_neighbors(grid, current.point) {
            if closed_set.contains_key(&neighbor) {
                continue;
            }

            let g_cost = current.g_cost + 1;

            if let Some(existing) = open_set.iter().find(|node| node.point == neighbor) {
                if g_cost < existing.g_cost {
                    continue;
                }
            }

            open_set.push(Node {
                point: neighbor,
                g_cost,
                h_cost: manhattan_distance(neighbor, goal),
                parent: Some(current.point),
            });
        }
    }

    None
}

fn get_neighbors(grid: &[Vec<bool>], point: Point) -> Vec<Point> {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut neighbors = Vec::new();

    for &(dx, dy) in &directions {
        let nx = point.x as isize + dx;
        let ny = point.y as isize + dy;

        if nx >= 0 && ny >= 0 {
            let nx = nx as usize;
            let ny = ny as usize;

            if nx < grid.len() && ny < grid[0].len() && grid[nx][ny] {
                neighbors.push(Point { x: nx, y: ny });
            }
        }
    }

    neighbors
}

fn manhattan_distance(p1: Point, p2: Point) -> usize {
    ((p1.x as isize - p2.x as isize).abs() + (p1.y as isize - p2.y as isize).abs()) as usize
}

fn main() {
    let grid = vec![
        vec![true, true, true, true, true],
        vec![true, false, false, false, true],
        vec![true, true, true, false, true],
        vec![true, false, true, true, true],
        vec![true, true, true, true, true],
    ];

    let start = Point { x: 0, y: 0 };
    let goal = Point { x: 4, y: 4 };

    if let Some(path) = astar(&grid, start, goal) {
        println!("Jalur ditemukan!");
        for step in path {
            println!("({}, {})", step.x, step.y);
        }
    } else {
        println!("Tidak ada jalur yang ditemukan.");
    }
}
