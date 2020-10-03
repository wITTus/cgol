use rand::Rng;

pub struct Field {
    cells: Vec<bool>,
    cells_age: Vec<u32>,
    rows: usize,
    columns: usize,
    iterations: usize,
}

impl Field {
    pub fn from_random(rows: usize, columns: usize) -> Field {
        let mut rng = rand::thread_rng();
        let cells = (0..columns * rows).map(|_| rng.gen_bool(0.5)).collect::<Vec<bool>>();
        let cells_age = vec![0; rows * columns];

        Field { cells, cells_age, rows, columns, iterations: 0 }
    }

    pub fn from_string(lines: Vec<&str>, rows: usize, columns: usize) -> Field {
        let mut cells = vec![false; rows * columns];
        let cells_age = vec![0; rows * columns];

        for (y, &line) in lines.iter().enumerate() {
            for (x, alive) in line.chars().enumerate() {
                cells[x + y * columns] = alive == 'O';
            }
        }

        Field { cells, cells_age, rows, columns, iterations: 0 }
    }

    pub fn next_iteration(&mut self) {
        let view = self.cells.as_slice().chunks(self.columns).collect::<Vec<&[bool]>>();

        let neighbour_field = self.generate_neighbour_field(&view);
        let new_cells = self.apply_rules(neighbour_field);
        let new_cells_age = self.age(&new_cells);

        self.cells = new_cells;
        self.cells_age = new_cells_age;
        self.iterations += 1;
    }

    pub fn generate_neighbour_field(&self, view: &Vec<&[bool]>) -> Vec<usize> {
        self.cells.iter().enumerate().map(|(i, _)| neighbours(&view, i % self.columns, i / self.columns)).collect()
    }

    pub fn apply_rules(&self, neighbour_field: Vec<usize>) -> Vec<bool> {
        self.cells.iter().zip(neighbour_field).map(|(alive, neighbours)| match neighbours {
            2 => true & alive,
            3 => true,
            _ => false
        }).collect()
    }

    pub fn age(&self, new_cells: &Vec<bool>) -> Vec<u32> {
        self.cells.iter().zip(new_cells).enumerate().map(|(idx, state)| match state {
            (true, true) => self.cells_age[idx] + 1,
            (_, _) => 0,
        }).collect()
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        output += gfx_cls();
        output += gfx_hline(self.columns).as_str();
        output += "\n";
        for r in 0..self.rows {
            for c in 0..self.columns {
                let idx = r * self.columns + c;
                let alive = self.cells[idx];
                let age = self.cells_age[idx];
                let gfx = gfx_cell(alive, age);
                output += gfx.as_str();
            }
            output += "\n";
        }
        output += gfx_hline(self.columns).as_str();
        output += "\n";
        output += self.iterations.to_string().as_str();
        output
    }
}

fn neighbours(rows: &Vec<&[bool]>, x: usize, y: usize) -> usize {
    let r = rows.len();
    let c = rows[0].len();

    let xm1 = (x as i32 - 1).rem_euclid(c as i32) as usize;
    let ym1 = (y as i32 - 1).rem_euclid(r as i32) as usize;
    let xp1 = (x as i32 + 1).rem_euclid(c as i32) as usize;
    let yp1 = (y as i32 + 1).rem_euclid(r as i32) as usize;

    [
        (xm1, ym1), (x, ym1), (xp1, ym1),
        (xm1, y), /*       */ (xp1, y),
        (xm1, yp1), (x, yp1), (xp1, yp1)
    ]
        .iter()
        .map(|(x, y)| rows[*y][*x])
        .filter(|i| { matches!(i, true) })
        .count()
}

const fn gfx_cls() -> &'static str {
    "\x1B[2J\x1B[2J\x1B[1;1H"
}

fn gfx_cell(alive: bool, age: u32) -> String {
    match alive {
        true => match age {
            0 => String::from("\x1B[38;5;34m\u{2588}"),
            1 => String::from("\x1B[38;5;35m\u{2588}"),
            2 => String::from("\x1B[38;5;36m\u{2588}"),
            3 => String::from("\x1B[38;5;37m\u{2588}"),
            4 => String::from("\x1B[38;5;38m\u{2588}"),
            5 => String::from("\x1B[38;5;39m\u{2588}"),
            _ => String::from("\x1B[38;5;21m\u{2588}")
        },
        false => String::from(" ")
    }
}

fn gfx_hline(columns: usize) -> String {
    "\x1B[38;5;15m".to_string() + "\u{25AC}".repeat(columns).as_str()
}

#[cfg(test)]
mod tests {
    use crate::field::neighbours;

    #[test]
    fn test_neighbours() {
        {
            let r: Vec<&[bool]> = vec!(&[true, true, true], &[false, false, false], &[true, true, true]);
            let n = neighbours(&r, 1, 1);
            assert_eq!(6, n);
        }
        {
            let r: Vec<&[bool]> = vec!(&[true, false, true], &[false, true, false], &[true, false, true]);
            let n = neighbours(&r, 1, 1);
            assert_eq!(4, n);
        }
        {
            let r: Vec<&[bool]> = vec!(&[false, false, false], &[false, true, false], &[false, false, false]);
            let n = neighbours(&r, 1, 1);
            assert_eq!(0, n);
        }
        {
            let r: Vec<&[bool]> = vec!(&[true, false, false], &[true, false, false], &[true, false, false]);
            let n = neighbours(&r, 1, 1);
            assert_eq!(3, n);
        }
        {
            let r: Vec<&[bool]> = vec!(&[true, true, true], &[true, false, true], &[false, true, false]);
            let n = neighbours(&r, 0, 0);
            assert_eq!(5, n);
        }
        {
            let r: Vec<&[bool]> = vec!(&[true, true, false], &[true, false, false], &[false, false, false]);
            let n = neighbours(&r, 2, 2);
            assert_eq!(3, n);
        }
        {
            let r: Vec<&[bool]> = vec!(&[true, true, false], &[false, false, false], &[false, false, false], &[true, true, true]);
            let n = neighbours(&r, 0, 0);
            assert_eq!(4, n);
        }
    }
}