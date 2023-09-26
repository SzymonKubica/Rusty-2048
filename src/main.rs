use rand::Rng;
use std::fmt;
#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Option<i32>>>,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Grid {
    fn empty() -> Grid {
        Grid {
            cells: vec![vec![None; 4]; 4],
        }
    }

    fn generate_new_tile(&mut self) {
        let mut rng = rand::thread_rng();
        let value: i32 = if rand::random() { 2 } else { 4 };
        loop {
            let x = rng.gen::<usize>() % 4;
            let y = rng.gen::<usize>() % 4;
            if let None = self.cells[x][y] {
                self.cells[x][y] = Some(value);
                break;
            }
        }
    }

    fn flush(&mut self, direction: Direction) {
        match direction {
            Direction::Up => todo!(),
            Direction::Down => todo!(),
            Direction::Left => todo!(),
            Direction::Right => todo!(),
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid_view: String = String::from("");
        for row in &self.cells {
            grid_view.push_str("\n");
            grid_view.push_str(line());
            grid_view.push_str("\n|");
            for cell in row {
                if let Some(value) = cell {
                    grid_view.push_str(format!("{: >4}", &value.to_string()).as_str());
                } else {
                    grid_view.push_str(empty());
                }
                grid_view.push_str("|");
            }
        }
        grid_view.push_str("\n");
        grid_view.push_str(line());
        write!(f, "{}", grid_view)
    }
}
fn line() -> &'static str {
    "---------------------"
}
fn empty() -> &'static str {
    "    "
}

fn main() {
    let mut grid = Grid::empty();
    print!("{}", grid);

    for _ in 1..10 {
        print!("\n");
        grid.generate_new_tile();
        print!("{}", grid);
    }
}
