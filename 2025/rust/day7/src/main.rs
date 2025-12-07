use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Position = (usize, usize);

fn count_hits(grid: &Vec<Vec<char>>, positions: &[Position]) -> usize {
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut queue: VecDeque<Position> = VecDeque::new();
    let mut visited: HashSet<Position> = HashSet::new();
    let mut hits: HashSet<Position> = HashSet::new();
    let mut count: usize = 0;

    for &start_position in positions {
        if visited.insert(start_position) {
            queue.push_back(start_position);
        }
    }

    while let Some(current_position) = queue.pop_front() {
        let (initial_r, c) = current_position;
        let mut r = initial_r + 1;

        while r < nrows {
            let current_position: Position = (r, c);

            if grid[current_position.0][current_position.1] == '^' {
                if hits.insert(current_position) {
                    count += 1;
                }

                if c > 0 {
                    let neighbor_left = (r, c - 1);
                    if visited.insert(neighbor_left) {
                        queue.push_back(neighbor_left);
                    }
                }

                if c + 1 < ncols {
                    let neighbor_right = (r, c + 1);
                    if visited.insert(neighbor_right) {
                        queue.push_back(neighbor_right);
                    }
                }
                break;
            }
            r += 1;
        }
    }
    count
}

fn count_timelines(grid: &Vec<Vec<char>>, positions: &[Position]) -> usize {
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut memory: HashMap<Position, usize> = HashMap::new();

    let mut count = 0;
    for &start_pos in positions {
        count += dfs(grid, start_pos.0, start_pos.1, nrows, ncols, &mut memory);
    }
    count
}

fn dfs(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    nrows: usize,
    ncols: usize,
    memory: &mut HashMap<Position, usize>,
) -> usize {
    let position = (row, col);

    if let Some(&cached) = memory.get(&position) {
        return cached;
    }

    if row == nrows - 1 {
        memory.insert(position, 1);
        return 1;
    }

    let mut current_row = row + 1;

    while current_row < nrows {
        if grid[current_row][col] == '^' {
            let mut timelines = 0;

            if col > 0 {
                timelines += dfs(grid, current_row, col - 1, nrows, ncols, memory);
            }
            if col + 1 < ncols {
                timelines += dfs(grid, current_row, col + 1, nrows, ncols, memory);
            }

            memory.insert(position, timelines);
            return timelines;
        }
        current_row += 1;
    }

    memory.insert(position, 1);
    1
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?.trim().to_string();
        if line.is_empty() {
            continue;
        }

        grid.push(line.chars().collect());
    }

    let positions: Vec<Position> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, &ch)| if ch == 'S' { Some((r, c)) } else { None })
        })
        .collect();

    // Part 1
    let answer1: usize = count_hits(&grid, &positions);
    println!("Part 1: {}", answer1);

    // Part 2
    let answer2: usize = count_timelines(&grid, &positions);
    println!("Part 2: {}", answer2);

    Ok(())
}
