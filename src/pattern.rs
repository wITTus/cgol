use std::{fs, io};
use std::ffi::OsStr;
use std::path::Path;

use pest::Parser;

#[derive(Parser)]
#[grammar = "../rle.pest"]
struct RleParser;


#[derive(Clone)]
pub struct Pattern {
    pub cells: Vec<bool>,
    pub rows: usize,
    pub columns: usize,
}


impl Pattern {
    pub fn from_file(filepath: &str) -> io::Result<Pattern> {
        let raw = fs::read_to_string(filepath)?;

        let pattern = match Path::new(filepath).extension().and_then(OsStr::to_str).unwrap() {
            "cells" => Pattern::from_cells(raw.as_str()),
            "rle" => Pattern::from_rle(raw.as_str()),
            unknown => panic!(".{} file support not implemented", unknown)
        };

        Ok(pattern)
    }

    pub fn from_cells(pattern: &str) -> Pattern {
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

        Pattern { cells, rows, columns }
    }

    pub fn from_rle(pattern: &str) -> Pattern {
        let pairs = RleParser::parse(Rule::doc, pattern).unwrap_or_else(|e| panic!("{}", e));

        let mut rows = 0;
        let mut columns = 0;

        for pair in pairs.clone() {
            match pair.as_rule() {
                Rule::config => {
                    for p in pair.into_inner() {
                        match p.as_rule() {
                            Rule::x_expr => columns = p.into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                            Rule::y_expr => rows = p.into_inner().next().unwrap().as_str().parse::<usize>().unwrap(),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        let mut cells = vec![false; rows * columns];
        for pair in pairs {
            match pair.as_rule() {
                Rule::pattern => {
                    for p in pair.into_inner() {
                        match p.as_rule() {
                            Rule::seq => {
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

                                let mut r = 0usize;
                                let mut c = 0usize;
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
                            }
                            _ => {}
                        };
                    }
                }
                _ => {}
            }
        }
        Pattern { cells, rows, columns }
    }
}

#[cfg(test)]
mod tests {
    use crate::field::Field;
    use crate::pattern::Pattern;

    #[test]
    fn test_rle() {
        let s = include_str!("../patterns/blinkerpuffer2.rle");
        let p = Pattern::from_rle(s);
        println!("{}", Field::from(p).to_string());
    }
}
