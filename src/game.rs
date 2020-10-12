use crate::field::{Field, wrap};
use crate::term::{colormap_gb, gfx_cell, gfx_cell_highres, gfx_hline, gfx_hline_highres, gfx_pos1};

pub struct Game {
    pub(crate) field: Field<bool>,
    ages: Field<u32>,
    marked: Field<bool>,
    iterations: usize,
}

impl Game {
    pub fn new(field: Field<bool>) -> Game {
        let ages = Field::with_size(field.rows, field.columns);
        let marked = Field::with_size(field.rows, field.columns);
        let iterations = 0;
        Game { field, ages, marked, iterations }
    }

    pub fn with_size(rows: usize, columns: usize) -> Game {
        let field = Field::with_size(rows, columns);
        Game::new(field)
    }

    pub fn from_random(rows: usize, columns: usize) -> Game {
        let field = Field::from_random(rows, columns);
        Game::new(field)
    }

    pub fn next_iteration(&mut self) {
        let cells_2d = self.field.proj2d();
        let neighbours = self.field.calculate_neighbours(&cells_2d);
        let new_cells = self.apply_rules(neighbours);
        let ages = self.calculate_ages(&new_cells);

        self.marked = Field::with_size(self.field.rows, self.field.columns);
        self.field = Field::new(new_cells, self.field.rows, self.field.columns);
        self.ages = Field::new(ages, self.field.rows, self.field.columns);
        self.iterations += 1;
    }

    pub fn mark_pattern(&mut self, pattern: &Field<bool>) {
        let cells_2d = self.field.proj2d();
        let pattern_2d = pattern.proj2d();

        let mut matches: Vec<(usize, usize)> = Vec::new();
        for r in 0..self.field.rows {
            for c in 0..self.field.columns {
                let mut matching_cells = 0;
                'p: for rr in 0..pattern.rows {
                    for cc in 0..pattern.columns {
                        let rrr = wrap(r, rr as i32, self.field.rows);
                        let ccc = wrap(c, cc as i32, self.field.columns);
                        if cells_2d[rrr][ccc] != pattern_2d[rr][cc] {
                            break 'p;
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
                    let rrr = wrap(r, rr as i32, self.field.rows);
                    let ccc = wrap(c, cc as i32, self.field.columns);
                    let idx = rrr * self.field.columns + ccc;
                    self.marked.cells[idx] = self.field.cells[idx] & true;
                    //println!("Marking {} {} WITH {}", rrr, ccc, self.cells[idx] & true);
                }
            }
            //println!("Found match at {},{}", r, c);
        }
    }

    pub fn apply_rules(&self, neighbour_field: Vec<usize>) -> Vec<bool> {
        self.field.cells.iter().zip(neighbour_field).map(|(alive, neighbours)| match neighbours {
            2 => true & alive,
            3 => true,
            _ => false
        }).collect()
    }

    pub fn calculate_ages(&self, new_cells: &[bool]) -> Vec<u32> {
        self.field.cells.iter().zip(new_cells).enumerate().map(|(idx, state)| match state {
            (true, true) => self.ages.cells[idx] + 1,
            (_, _) => 0,
        }).collect()
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        let hline = gfx_hline(self.field.columns);
        output += gfx_pos1();
        output += hline.as_str();
        output += "\n";
        //output += "\x1B[38;5;1m  012345678901234567890\n";
        for r in 0..self.field.rows {
            //output += format!("{:0w$}", r.to_string(), w=2).as_str();
            for c in 0..self.field.columns {
                let idx = r * self.field.columns + c;
                let alive = self.field.cells[idx];
                let age = self.ages.cells[idx];
                let color = if self.marked.cells[idx] { "\x1B[38;5;1m".to_string() } else { colormap_gb(age) };
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
        let hline = gfx_hline_highres(self.field.columns);
        output += gfx_pos1();
        output += hline.as_str();
        output += "\n";
        for r in (0..self.field.rows).step_by(2) {
            for c in (0..self.field.columns).step_by(2) {
                let index = |r, c| r * self.field.columns + c;
                let idxul = index(r, c);
                let idxur = idxul + 1;
                let idxbl = index(r + 1, c);
                let idxbr = idxbl + 1;

                let alive_ul = *self.field.cells.get(idxul).unwrap();
                let alive_ur = *self.field.cells.get(idxur).unwrap_or(&false);
                let alive_bl = *self.field.cells.get(idxbl).unwrap_or(&false);
                let alive_br = *self.field.cells.get(idxbr).unwrap_or(&false);

                let age_ul = *self.ages.cells.get(idxul).unwrap();
                let age_ur = *self.ages.cells.get(idxur).unwrap_or(&0);
                let age_bl = *self.ages.cells.get(idxbl).unwrap_or(&0);
                let age_br = *self.ages.cells.get(idxbr).unwrap_or(&0);

                let age = (age_ul + age_ur + age_bl + age_br) / 4;
                let color = if self.marked.cells[idxul] { "\x1B[38;5;1m".to_string() } else { colormap_gb(age) };
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

impl From<Field<bool>> for Game {
    fn from(field: Field<bool>) -> Self {
        Game::new(field)
    }
}

#[cfg(test)]
mod tests {
    use crate::field::{Field};
    use crate::game::{Game};


    #[test]
    fn test_output_highres() {
        {
            let glider = Field::from_cells("\
......
..O...
...O..
.OOO..");

            let game = Game::from(glider);
            println!("{}", game.to_string_highres());
        }
    }

    #[test]
    fn test_find_pattern() {
        {
            let glider = Field::from_cells("\
.O.
..O
OOO");

            let scene = Field::from_cells("\
OO..OOO..
O........
OOOOO....
.......OO
..OO..O..
.......O.
.....OOO.
.........
");

            let mut game = Game::from(scene);
            game.mark_pattern(&glider);
            println!("{}", game.to_string());
        }
    }
}