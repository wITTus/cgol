use rand::Rng;

pub struct Field {
    cells: Vec<bool>,
    rows: usize,
    columns: usize,
    iterations: usize,
}

impl Field {
    pub fn from_random(rows: usize, columns: usize) -> Field {
        let mut rng = rand::thread_rng();
        let cells = (0..columns * rows).map(|_| rng.gen_bool(0.5)).collect::<Vec<bool>>();

        Field {
            cells,
            rows,
            columns,
            iterations: 0,
        }
    }

    pub fn from_string(lines: Vec<&str>, rows: usize, columns: usize) -> Field {
        let mut cells = Vec::with_capacity(rows * columns);
        cells.resize(rows * columns, false);

        for (y, &line) in lines.iter().enumerate() {
            for (x, alive) in line.chars().enumerate() {
                cells[x + y * columns] = alive == 'O';
            }
        }

        Field {
            cells,
            rows,
            columns,
            iterations: 0,
        }
    }

    pub fn apply_rules(&mut self) {
        let view = self.cells.as_slice().chunks(self.columns).collect::<Vec<&[bool]>>();
        let cells = self.cells.iter().enumerate()
            .map(|(i, alive)| match neighbours(&view, i % self.columns, i / self.columns) {
                2 => true & alive,
                3 => true,
                _ => false
            }).collect();

        self.cells = cells;
        self.iterations += 1;
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        output += "\x1B[2J\x1B[2J\x1B[1;1H";
        output += "\u{25AC}".repeat(self.columns as usize).as_str();
        output += "\n";
        for r in 0..self.rows {
            for c in 0..self.columns {
                let b = self.cells[(r * self.columns + c) as usize];
                let c = match b {
                    true => "\u{2588}",
                    false => " "
                };
                output += c;
            }
            output += "\n";
        }
        output += "\u{25AC}".repeat(self.columns as usize).as_str();
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