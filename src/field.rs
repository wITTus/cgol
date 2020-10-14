use std::{fs, io};
use std::cmp::min;
use std::ffi::OsStr;
use std::path::Path;

use itertools::Itertools;
use pest::Parser;
use rand::Rng;

use crate::rule::AutomataRule;

#[derive(Parser)]
#[grammar = "../rle.pest"]
struct RleParser;

#[derive(Clone)]
pub struct Field<T> {
    pub cells: Vec<T>,
    pub rows: usize,
    pub columns: usize,
}

impl<T> Field<T> {
    pub fn new(cells: Vec<T>, rows: usize, columns: usize) -> Field<T> {
        Field { cells, rows, columns }
    }

    pub fn with_size(rows: usize, columns: usize) -> Field<T>
        where T: Default + Copy
    {
        let cells = vec![T::default(); rows * columns];
        Field::new(cells, rows, columns)
    }

    pub fn insert(&mut self, pattern: Field<T>)
        where T: Copy
    {
        let pattern_2d = pattern.proj2d();

        for r in 0..min(pattern.rows, self.rows) {
            for c in 0..min(pattern.columns, self.columns) {
                self.cells[r * self.columns + c] = pattern_2d[r][c];
            }
        }
    }

    pub fn proj2d(&self) -> Vec<&[T]> {
        self.cells.chunks(self.columns).collect::<Vec<&[T]>>()
    }

    pub fn find_pattern(&self, pattern: &Field<T>) -> Vec<(usize, usize)>
        where T: Eq
    {
        let cells_2d = self.proj2d();
        let pattern_2d = pattern.proj2d();

        let mut matches: Vec<(usize, usize)> = Vec::new();
        for r in 0..self.rows {
            for c in 0..self.columns {
                let mut matching_cells = 0;
                'p: for rr in 0..pattern.rows {
                    for cc in 0..pattern.columns {
                        let rrr = wrap(r, rr as i32, self.rows);
                        let ccc = wrap(c, cc as i32, self.columns);
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
        matches
    }
}

impl Field<bool> {
    pub fn from_random(rows: usize, columns: usize) -> Field<bool> {
        let mut rng = rand::thread_rng();
        let cells = (0..columns * rows).map(|_| rng.gen_bool(0.05)).collect::<Vec<bool>>();

        Field::new(cells, rows, columns)
    }

    pub fn from_normal_distribution(rows: usize, columns: usize) -> Field<bool> {
        let x0 = columns as f64 / 2.0;
        let y0 = rows as f64 / 2.0;
        let sx = columns as f64 / 10.0;
        let sy = rows as f64 / 10.0;
        let p = |r, c| gaussian_2d(c, r, x0, y0, sx, sy);

        let mut rng = rand::thread_rng();
        let cells = (0..rows).cartesian_product(0..columns)
            .map(|(r, c)| rng.gen_bool(p(r as f64, c as f64)))
            .collect::<Vec<bool>>();

        Field::new(cells, rows, columns)
    }

    pub fn from_file(filepath: &str) -> io::Result<Field<bool>> {
        let raw = fs::read_to_string(filepath)?;

        let field = match Path::new(filepath).extension().and_then(OsStr::to_str).unwrap() {
            "cells" => Field::from_cells(raw.as_str()),
            "rle" => Field::from_rle(raw.as_str()),
            unknown => panic!(".{} file support not implemented", unknown)
        };

        Ok(field)
    }

    pub fn from_cells(pattern: &str) -> Field<bool> {
        let lines: Vec<&str> = pattern.lines()
            .filter(|&l| !l.starts_with('!'))
            .map(|l| l.trim_end())
            .collect();

        let rows = lines.len();
        let columns = lines.iter().map(|&l| l.len()).max().expect("Couldn't read pattern file");

        let mut cells = vec![false; rows * columns];

        for (y, &line) in lines.iter().enumerate() {
            for (x, alive) in line.chars().enumerate() {
                cells[x + y * columns] = alive == 'O';
            }
        }

        Field { cells, rows, columns }
    }

    pub fn from_rle(pattern: &str) -> Field<bool> {
        let pairs = RleParser::parse(Rule::doc, pattern).unwrap_or_else(|e| panic!("{}", e));

        let mut rows = 0;
        let mut columns = 0;

        for pair in pairs.clone() {
            if let Rule::config = pair.as_rule() {
                for p in pair.into_inner() {
                    match p.as_rule() {
                        Rule::x_expr => columns = p.into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        Rule::y_expr => rows = p.into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                        _ => {}
                    }
                }
            }
        }

        let mut cells = vec![false; rows * columns];
        for pair in pairs {
            if let Rule::pattern = pair.as_rule() {
                let mut r = 0usize;
                let mut c = 0usize;

                for p in pair.into_inner() {
                    if let Rule::seq = p.as_rule() {
                        let mut it = p.into_inner();
                        let first = it.next().unwrap();
                        let second = it.next();
                        let (n, tag) = match first.as_rule() {
                            Rule::number => {
                                let n = first.as_str().parse::<usize>().unwrap();
                                let t = second.unwrap().as_str();
                                (n, t)
                            }
                            Rule::tag => {
                                let n = 1;
                                let t = first.as_str();
                                (n, t)
                            }
                            _ => unreachable!()
                        };

                        tag.repeat(n).chars().for_each(|t| match t {
                            '$' => {
                                r += 1;
                                c = 0;
                            }
                            any => {
                                let pos = c + r * columns;
                                cells[pos] = any == 'o';
                                c += 1;
                            }
                        });
                    };
                }
            }
        }
        Field { cells, rows, columns }
    }

    pub fn calculate_neighbours(&self, cells_2d: &[&[bool]]) -> Vec<usize> {
        self.cells.iter().enumerate().map(|(i, _)| neighbours(&cells_2d, i % self.columns, i / self.columns)).collect()
    }

    pub fn apply_rule(&self, neighbour_field: Vec<usize>, rule: &AutomataRule) -> Vec<bool> {
        self.cells.iter().zip(neighbour_field).map(|(&alive, neighbours)| rule.apply(alive, neighbours)).collect()
    }
}

pub fn wrap(pos: usize, delta: i32, lim: usize) -> usize {
    (pos as i32 + delta).rem_euclid(lim as i32) as usize
}

fn gaussian_2d(x: f64, y: f64, x0: f64, y0: f64, sx: f64, sy: f64) -> f64 {
    let two_sigma_sq_x = 2.0 * sx * sx;
    let two_sigma_sq_y = 2.0 * sy * sy;
    (-((x - x0).powi(2) / two_sigma_sq_x + (y - y0).powi(2) / two_sigma_sq_y)).exp()
}

fn neighbours(cells_2d: &[&[bool]], x: usize, y: usize) -> usize {
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

impl<T: PartialEq> PartialEq for Field<T> {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.columns == other.columns && self.cells == other.cells
    }
}

#[cfg(test)]
mod tests {
    use crate::field::{Field, neighbours};
    use crate::game::Game;
    use crate::rule::AutomataRule;

    #[test]
    fn test_rle() {
        let s = include_str!("../patterns/blinkerpuffer2.rle");
        let p = Field::from_rle(s);
        let ss = format!("{}", Game::new(p, AutomataRule::cgol()).to_string());

        assert_eq!("\u{1b}[1;1H\u{1b}[38;5;15m▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬\n             \u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█ \n            \u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█\n           \u{1b}[38;5;34m█\u{1b}[38;5;34m█ \u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█\n            \u{1b}[38;5;34m█\u{1b}[38;5;34m█   \n                 \n                 \n         \u{1b}[38;5;34m█ \u{1b}[38;5;34m█     \n  \u{1b}[38;5;34m█     \u{1b}[38;5;34m█  \u{1b}[38;5;34m█     \n \u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█   \u{1b}[38;5;34m█ \u{1b}[38;5;34m█     \n\u{1b}[38;5;34m█\u{1b}[38;5;34m█   \u{1b}[38;5;34m█\u{1b}[38;5;34m█ \u{1b}[38;5;34m█\u{1b}[38;5;34m█       \n \u{1b}[38;5;34m█       \u{1b}[38;5;34m█       \n  \u{1b}[38;5;34m█\u{1b}[38;5;34m█  \u{1b}[38;5;34m█  \u{1b}[38;5;34m█       \n          \u{1b}[38;5;34m█      \n  \u{1b}[38;5;34m█\u{1b}[38;5;34m█  \u{1b}[38;5;34m█  \u{1b}[38;5;34m█       \n \u{1b}[38;5;34m█       \u{1b}[38;5;34m█       \n\u{1b}[38;5;34m█\u{1b}[38;5;34m█   \u{1b}[38;5;34m█\u{1b}[38;5;34m█ \u{1b}[38;5;34m█\u{1b}[38;5;34m█       \n \u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█   \u{1b}[38;5;34m█ \u{1b}[38;5;34m█     \n  \u{1b}[38;5;34m█     \u{1b}[38;5;34m█  \u{1b}[38;5;34m█     \n         \u{1b}[38;5;34m█ \u{1b}[38;5;34m█     \n                 \n                 \n            \u{1b}[38;5;34m█\u{1b}[38;5;34m█   \n           \u{1b}[38;5;34m█\u{1b}[38;5;34m█ \u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█\n            \u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█\n             \u{1b}[38;5;34m█\u{1b}[38;5;34m█\u{1b}[38;5;34m█ \n\u{1b}[38;5;15m▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬\n0", ss);
    }

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