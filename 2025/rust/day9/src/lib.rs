pub mod parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
}

impl Tile {
    pub fn new(x: i32, y: i32) -> Self {
        Tile { x, y }
    }
}

pub fn largest_red_red_rect(tiles: &[Tile]) -> i64 {
    let mut max_area: i64 = 0;
    let number_of_tiles = tiles.len();

    for i in 0..number_of_tiles {
        for j in (i + 1)..number_of_tiles {
            let t1 = tiles[i];
            let t2 = tiles[j];

            if t1.x == t2.x || t1.y == t2.y {
                continue;
            }

            let w = (t1.x - t2.x).abs() as i64 + 1;
            let h = (t1.y - t2.y).abs() as i64 + 1;
            let area = w * h;
            max_area = max_area.max(area);
        }
    }

    max_area
}

pub fn largest_red_green_rect(tiles: &[Tile]) -> i64 {
    if tiles.len() < 2 {
        return 0;
    }

    let mut lines = Vec::new();
    for i in 0..tiles.len() {
        let next = (i + 1) % tiles.len();
        lines.push((tiles[i], tiles[next]));
    }

    let mut rectangles = Vec::new();
    let number_of_tiles = tiles.len();

    for i in 0..number_of_tiles {
        for j in (i + 1)..number_of_tiles {
            let t1 = tiles[i];
            let t2 = tiles[j];

            if t1.x == t2.x || t1.y == t2.y {
                continue;
            }

            let w = (t1.x - t2.x).abs() as i64 + 1;
            let h = (t1.y - t2.y).abs() as i64 + 1;
            let area = w * h;

            rectangles.push((t1, t2, area));
        }
    }

    rectangles.sort_by(|a, b| b.2.cmp(&a.2));

    for (a, b, area) in rectangles {
        let rect_intersects_line = lines.iter().any(|(line_start, line_end)| {
            let rect_min_x = a.x.min(b.x);
            let rect_max_x = a.x.max(b.x);
            let rect_min_y = a.y.min(b.y);
            let rect_max_y = a.y.max(b.y);

            let line_min_x = line_start.x.min(line_end.x);
            let line_max_x = line_start.x.max(line_end.x);
            let line_min_y = line_start.y.min(line_end.y);
            let line_max_y = line_start.y.max(line_end.y);

            let left_of_rect = rect_max_x <= line_min_x;
            let right_of_rect = rect_min_x >= line_max_x;
            let above = rect_max_y <= line_min_y;
            let below = rect_min_y >= line_max_y;

            !(left_of_rect || right_of_rect || above || below)
        });

        if !rect_intersects_line {
            return area;
        }
    }

    0
}
