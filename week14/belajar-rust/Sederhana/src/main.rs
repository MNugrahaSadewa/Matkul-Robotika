use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)] // Tambahkan PartialEq di sini
struct Point {
    x: usize,
    y: usize,
}

fn is_valid_move(x: isize, y: isize, grid: &[Vec<u8>], visited: &[Vec<bool>]) -> bool {
    x >= 0
        && y >= 0
        && x < grid.len() as isize
        && y < grid[0].len() as isize
        && grid[x as usize][y as usize] == 0
        && !visited[x as usize][y as usize]
}

fn bfs_pathfinding(grid: &[Vec<u8>], start: Point, goal: Point) -> Option<Vec<Point>> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut parent = vec![vec![None; grid[0].len()]; grid.len()];

    queue.push_back(start);
    visited[start.x][start.y] = true;

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some(current) = queue.pop_front() {
        if current == goal {
            // Rekonstruksi jalur dari tujuan ke awal
            let mut path = Vec::new();
            let mut current = Some(goal);

            while let Some(c) = current {
                path.push(c);
                current = parent[c.x][c.y];
            }

            path.reverse();
            return Some(path);
        }

        for &(dx, dy) in &directions {
            let nx = current.x as isize + dx;
            let ny = current.y as isize + dy;

            if is_valid_move(nx, ny, grid, &visited) {
                let next = Point {
                    x: nx as usize,
                    y: ny as usize,
                };

                queue.push_back(next);
                visited[next.x][next.y] = true;
                parent[next.x][next.y] = Some(current);
            }
        }
    }

    None // Tidak ada jalur ditemukan
}

fn main() {
    let grid = vec![
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
    ];

    let start = Point { x: 0, y: 0 };
    let goal = Point { x: 4, y: 4 };

    if let Some(path) = bfs_pathfinding(&grid, start, goal) {
        println!("Jalur ditemukan:");
        for point in path {
            println!("({}, {})", point.x, point.y);
        }
    } else {
        println!("Tidak ada jalur yang ditemukan.");
    }
}
