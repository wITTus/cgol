use std::{fs, io};

#[derive(Clone)]
pub struct Pattern {
    pub cells: Vec<bool>,
    pub rows: usize,
    pub columns: usize,
}

impl Pattern {
    pub fn from_file(filepath: &str) -> io::Result<Pattern> {
        let raw = fs::read_to_string(filepath)?;
        Ok(Pattern::from_string(raw.as_str()))
    }

    pub fn from_string(pattern: &str) -> Pattern {
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
}