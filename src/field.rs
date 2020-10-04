use rand::Rng;

use crate::pattern::Pattern;
use crate::gfx::{gfx_pos1, gfx_hline, gfx_cell, colormap_gb, gfx_hline_highres, gfx_cell_highres};

pub struct Field {
    cells: Vec<bool>,
    ages: Vec<u32>,
    rows: usize,
    columns: usize,
    iterations: usize,
    marked: Vec<bool>,
}

impl Field {
    pub fn new(cells: Vec<bool>, rows: usize, columns: usize) -> Field {
        let iterations = 0;
        let ages = vec![0; rows * columns];
        let marked = vec![false; rows * columns];

        Field { cells, ages, rows, columns, iterations, marked }
    }

    pub fn with_size(rows: usize, columns: usize) -> Field {
        let cells = vec![false; rows * columns];
        Field::new(cells, rows, columns)
    }

    pub fn from_random(rows: usize, columns: usize) -> Field {
        let mut rng = rand::thread_rng();
        let cells = (0..columns * rows).map(|_| rng.gen_bool(0.5)).collect::<Vec<bool>>();

        Field::new(cells, rows, columns)
    }

    pub fn insert(&mut self, pattern: Pattern) {
        let pattern_2d = proj2d(&pattern.cells, pattern.columns);

        for r in 0..pattern.rows {
            for c in 0..pattern.columns {
                self.cells[r * self.columns + c] = pattern_2d[r][c];
            }
        }
    }

    pub fn next_iteration(&mut self) {
        let cells_2d = proj2d(&self.cells, self.columns);

        let neighbours = self.calculate_neighbours(&cells_2d);
        let new_cells = self.apply_rules(neighbours);
        let ages = self.calculate_ages(&new_cells);

        self.marked = vec![false; self.rows * self.columns];
        self.cells = new_cells;
        self.ages = ages;
        self.iterations += 1;
    }

    pub fn mark_pattern(&mut self, pattern: &Pattern) {
        let cells_2d = proj2d(&self.cells, self.columns);
        let pattern_2d = proj2d(&pattern.cells, pattern.columns);

        let mut matches: Vec<(usize, usize)> = Vec::new();
        for r in 0..self.rows {
            for c in 0..self.columns {
                let mut matching_cells = 0;
                for rr in 0..pattern.rows {
                    for cc in 0..pattern.columns {
                        let rrr = wrap(r, rr as i32, self.rows);
                        let ccc = wrap(c, cc as i32, self.columns);
                        if cells_2d[rrr][ccc] != pattern_2d[rr][cc] {
                            break;
                        } else {
                            matching_cells += 1;
                        }
                    }
                }
                if matching_cells == pattern.rows * pattern.columns {
                    matches.push((r, c));
                }
            }
        }

        for (r, c) in matches {
            for rr in 0..pattern.rows {
                for cc in 0..pattern.columns {
                    let rrr = wrap(r, rr as i32, self.rows);
                    let ccc = wrap(c, cc as i32, self.columns);
                    let idx = rrr * self.columns + ccc;
                    self.marked[idx] = self.cells[idx] & true;
                    //println!("Marking {} {} WITH {}", rrr, ccc, self.cells[idx] & true);
                }
            }
            //println!("Found match at {},{}", r, c);
        }
    }

    pub fn calculate_neighbours(&self, cells_2d: &Vec<&[bool]>) -> Vec<usize> {
        self.cells.iter().enumerate().map(|(i, _)| neighbours(&cells_2d, i % self.columns, i / self.columns)).collect()
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
                let color = if self.marked[idx] { "\x1B[38;5;1m".to_string() } else { colormap_gb(age) };
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
                let color = if self.marked[idxul] { "\x1B[38;5;1m".to_string() } else { colormap_gb(age) };
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

impl From<Pattern> for Field {
    fn from(pattern: Pattern) -> Self {
        let mut field = Field::with_size(pattern.rows, pattern.columns);
        field.insert(pattern);
        field
    }
}

fn proj2d(cells: &Vec<bool>, columns: usize) -> Vec<&[bool]> {
    cells.as_slice().chunks(columns).collect::<Vec<&[bool]>>()
}

pub fn wrap(pos: usize, delta: i32, lim: usize) -> usize {
    (pos as i32 + delta).rem_euclid(lim as i32) as usize
}

fn neighbours(cells_2d: &Vec<&[bool]>, x: usize, y: usize) -> usize {
    let r = cells_2d.len();
    let c = cells_2d[0].len();

    let xm1 = wrap(x, -1, c);
    let ym1 = wrap(y, -1, r);
    let xp1 = wrap(x, 1, c);
    let yp1 = wrap(y, 1, r);

    [
        (xm1, ym1), (x, ym1), (xp1, ym1),
        (xm1, y), /*       */ (xp1, y),
        (xm1, yp1), (x, yp1), (xp1, yp1)
    ]
        .iter()
        .map(|(x, y)| cells_2d[*y][*x])
        .filter(|i| { matches!(i, true) })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::field::{Field, neighbours};
    use crate::pattern::Pattern;

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
            let glider = Pattern::from_string("\
......
..O...
...O..
.OOO..");

            let field = Field::from(glider);
            println!("{}", field.to_string_highres());
        }
    }

    #[test]
    fn test_find_pattern() {
        {
            let glider = Pattern::from_string("\
.O.
..O
OOO");

            let scene = Pattern::from_string("\
OO..OOO..
O........
OOOOO....
.......OO
......O..
.......O.
.....OOO.
.........
");

            let mut field = Field::from(scene);
            field.mark_pattern(&glider);
            println!("{}", field.to_string());
        }
    }
}