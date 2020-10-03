use std::{fs, io, thread, time};
use std::cmp::max;
use std::io::{stdout, Stdout, Write};

use clap::{App, Arg};

use crate::field::Field;

mod field;

fn main() {
    let matches = App::new("Conway's Game of Life").author("w177us")
        .about("Run with e.g. `cgol -c $COLUMNS -r $[ LINES-3 ] -i 30 -p patterns/glider.cells'")
        .arg(Arg::with_name("rows").short('r').about("Number of rows").takes_value(true))
        .arg(Arg::with_name("columns").short('c').about("Number of columns").takes_value(true))
        .arg(Arg::with_name("interval").short('i').about("Tick interval (in ms)").takes_value(true))
        .arg(Arg::with_name("pattern").short('p').about("Load pattern from file").takes_value(true))
        .get_matches();

    let rows = matches.value_of("rows").map(|v| v.parse::<usize>().unwrap()).unwrap_or(21);
    let columns = matches.value_of("columns").map(|v| v.parse::<usize>().unwrap()).unwrap_or(80);
    let interval = matches.value_of("interval").map(|v| v.parse::<u64>().unwrap()).unwrap_or(30);
    let pattern = matches.value_of("pattern").map(|p| load_pattern(p, rows, columns).expect("Couldn't open .cells file"));

    let mut stdout = stdout();
    let mut field = pattern.unwrap_or_else(|| Field::from_random(rows, columns));

    loop {
        print(&mut stdout, field.to_string().as_str());
        thread::sleep(time::Duration::from_millis(interval));
        field.next_iteration();
    }
}

fn load_pattern(pattern_file: &str, rows: usize, columns: usize) -> io::Result<Field> {
    let raw = fs::read_to_string(pattern_file)?;
    let lines: Vec<&str> = raw.lines()
        .into_iter()
        .filter(|&l| !l.starts_with("!"))
        .map(|l| l.trim_end())
        .collect();

    let req_rows = max(rows, lines.len());
    let req_columns = max(columns, lines.iter().map(|&l| l.len()).max().expect("Couldn't read pattern file"));

    Ok(Field::from_string(lines, req_rows, req_columns))
}

#[allow(unused_must_use)]
fn print(stdout: &mut Stdout, field: &str) {
    stdout.write(field.as_bytes());
    stdout.flush();
}
