use rand::Rng;

pub struct Field {
    cells: Vec<bool>,
    ages: Vec<u32>,
    rows: usize,
    columns: usize,
    iterations: usize
}

impl Field {
    pub fn new(cells: Vec<bool>, rows: usize, columns: usize) -> Field {
        let iterations = 0;
        let ages = vec![0; rows * columns];

        Field { cells, ages, rows, columns, iterations }
    }

    pub fn from_random(rows: usize, columns: usize) -> Field {
        let mut rng = rand::thread_rng();
        let cells = (0..columns * rows).map(|_| rng.gen_bool(0.5)).collect::<Vec<bool>>();

        Field::new(cells, rows, columns)
    }

    pub fn from_string(lines: Vec<&str>, rows: usize, columns: usize) -> Field {
        let mut cells = vec![false; rows * columns];

        for (y, &line) in lines.iter().enumerate() {
            for (x, alive) in line.chars().enumerate() {
                cells[x + y * columns] = alive == 'O';
            }
        }

        Field::new(cells, rows, columns)
    }

    pub fn next_iteration(&mut self) {
        let view2d = self.cells.as_slice().chunks(self.columns).collect::<Vec<&[bool]>>();

        let neighbours = self.calculate_neighbours(&view2d);
        let new_cells = self.apply_rules(neighbours);
        let ages = self.calculate_ages(&new_cells);

        self.cells = new_cells;
        self.ages = ages;
        self.iterations += 1;
    }

    pub fn calculate_neighbours(&self, view2d: &Vec<&[bool]>) -> Vec<usize> {
        self.cells.iter().enumerate().map(|(i, _)| neighbours(&view2d, i % self.columns, i / self.columns)).collect()
    }

    pub fn apply_rules(&self, neighbour_field: Vec<usize>) -> Vec<bool> {
        self.cells.iter().zip(neighbour_field).map(|(alive, neighbours)| match neighbours {
            2 => true & alive,
            3 => true,
            _ => false
        }).collect()
    }

    pub fn calculate_ages(&self, new_cells: &Vec<bool>) -> Vec<u32> {
        self.cells.iter().zip(new_cells).enumerate().map(|(idx, state)| match state {
            (true, true) => self.ages[idx] + 1,
            (_, _) => 0,
        }).collect()
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        let hline = gfx_hline(self.columns);
        output += gfx_pos1();
        output += hline.as_str();
        output += "\n";
        for r in 0..self.rows {
            for c in 0..self.columns {
                let idx = r * self.columns + c;
                let alive = self.cells[idx];
                let age = self.ages[idx];
                let color = color_by_age(age);
                let gfx = gfx_cell(alive, color);
                output += gfx.as_str();
            }
            output += "\n";
        }
        output += hline.as_str();
        output += "\n";
        output += self.iterations.to_string().as_str();
        output
    }

    pub fn to_string_highres(&self) -> String {
        let mut output = String::new();
        let hline = gfx_hline_highres(self.columns);
        output += gfx_pos1();
        output += hline.as_str();
        output += "\n";
        for r in (0..self.rows).step_by(2) {
            for c in (0..self.columns).step_by(2) {
                let index = |r, c| r * self.columns + c;
                let idxul = index(r, c);
                let idxur = idxul + 1;
                let idxbl = index(r + 1, c);
                let idxbr = idxbl + 1;

                let alive_ul = *self.cells.get(idxul).unwrap();
                let alive_ur = *self.cells.get(idxur).unwrap_or(&false);
                let alive_bl = *self.cells.get(idxbl).unwrap_or(&false);
                let alive_br = *self.cells.get(idxbr).unwrap_or(&false);

                let age_ul = *self.ages.get(idxul).unwrap();
                let age_ur = *self.ages.get(idxur).unwrap_or(&0);
                let age_bl = *self.ages.get(idxbl).unwrap_or(&0);
                let age_br = *self.ages.get(idxbr).unwrap_or(&0);

                let age = (age_ul + age_ur + age_bl + age_br) / 4;
                let color = color_by_age(age);
                let gfx = gfx_cell_highres(alive_ul, alive_ur, alive_bl, alive_br, color);
                output += gfx.as_str();
            }
            output += "\n";
        }
        output += hline.as_str();
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

pub const fn gfx_cls() -> &'static str {
    "\x1B[2J\x1B[1;1H"
}

pub const fn gfx_pos1() -> &'static str {
    "\x1B[1;1H"
}

fn gfx_cell(alive: bool, color: String) -> String {
    let s = "\u{2588}";

    match alive {
        true => color + s,
        false => String::from(" ")
    }
}

fn gfx_cell_highres(alive_ul: bool, alive_ur: bool, alive_bl: bool, alive_br: bool, color: String) -> String {
    let ws = " ".to_string();

    let symbol = match (alive_ul, alive_ur, alive_bl, alive_br) {
        (false, false, false, false) => ws.as_str(),
        (false, false, false, true) => "\u{2597}",
        (false, false, true, false) => "\u{2596}",
        (false, false, true, true) => "\u{2584}",
        (false, true, false, false) => "\u{259D}",
        (false, true, false, true) => "\u{2590}",
        (false, true, true, false) => "\u{259E}",
        (false, true, true, true) => "\u{259F}",
        (true, false, false, false) => "\u{2598}",
        (true, false, false, true) => "\u{259A}",
        (true, false, true, false) => "\u{258C}",
        (true, false, true, true) => "\u{2599}",
        (true, true, false, false) => "\u{2580}",
        (true, true, false, true) => "\u{259C}",
        (true, true, true, false) => "\u{259B}",
        (true, true, true, true) => "\u{2588}",
    };

    match symbol {
        " " => ws,
        s => color + &s
    }
}

fn gfx_hline(columns: usize) -> String {
    "\x1B[38;5;15m".to_string() + "\u{25AC}".repeat(columns).as_str()
}

fn gfx_hline_highres(columns: usize) -> String {
    "\x1B[38;5;15m".to_string() + "\u{25AC}".repeat(columns / 2).as_str()
}

fn color_by_age(age: u32) -> String {
    match age {
        0 => String::from("\x1B[38;5;34m"),
        1 => String::from("\x1B[38;5;35m"),
        2 => String::from("\x1B[38;5;36m"),
        3 => String::from("\x1B[38;5;37m"),
        4 => String::from("\x1B[38;5;38m"),
        5 => String::from("\x1B[38;5;39m"),
        _ => String::from("\x1B[38;5;21m")
    }
}

#[cfg(test)]
mod tests {
    use crate::field::{Field, neighbours};

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

    #[test]
    fn test_output_highres() {
        {
            let glider = "\
......\
..O...\
...O..\
.OOO..".lines();

            let field = Field::from_string(glider.collect::<Vec<&str>>(), 4, 6);
            println!("{}", field.to_string_highres());
        }
    }
}