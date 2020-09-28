use std::{thread, time};
use std::io::{stdout, Stdout, Write};

use clap::{App, Arg};
use rand::Rng;

fn main() {
    let matches = App::new("Conway's Game of Life").author("w177us")
        .about("Run with e.g. `cgol -c $COLUMNS -r $[ LINES-2 ] -i 30'")
        .arg(Arg::with_name("rows").short('r').about("Number of rows").takes_value(true))
        .arg(Arg::with_name("columns").short('c').about("Number of columns").takes_value(true))
        .arg(Arg::with_name("interval").short('i').about("Tick interval (in ms)").takes_value(true))
        .get_matches();
    let rows = matches.value_of("rows").map(|v| v.parse::<u32>().unwrap()).unwrap_or(24);
    let columns = matches.value_of("columns").map(|v| v.parse::<u32>().unwrap()).unwrap_or(80);
    let interval = matches.value_of("interval").map(|v| v.parse::<u64>().unwrap()).unwrap_or(50);

    let mut stdout = stdout();
    let mut rng = rand::thread_rng();
    let mut field = (0..columns * rows).map(|_| rng.gen_bool(0.5)).collect::<Vec<bool>>();

    let mut iteration_counter = 1;
    loop {
        print(&mut stdout, &field, rows, columns,iteration_counter);
        thread::sleep(time::Duration::from_millis(interval));
        field = apply_rules(field, rows, columns);
        iteration_counter += 1;
    }
}

fn apply_rules(field: Vec<bool>, rows: u32, columns: u32) -> Vec<bool> {
    field.iter().enumerate()
        .map(|(i, alive)| match neighbours(&field, i as i32, rows as i32, columns as i32) {
            2 => true & alive,
            3 => true,
            _ => false
        }).collect()
}

fn neighbours(m: &Vec<bool>, index: i32, rows: i32, columns: i32) -> usize {
    [
        index - columns - 1, index - columns, index - columns + 1,
        index - 1, /*                      */ index + 1,
        index + columns - 1, index + columns, index + columns + 1
    ]
        .iter()
        .map(|&idx| match idx {
            i if i < 0 => false,
            i if i >= columns * rows => false,
            i => *m.get(i as usize).expect("Lookup failed")
        })
        .filter(|i| { matches!(i, true) })
        .count()
}

#[allow(unused_must_use)]
fn print(stdout: &mut Stdout, map: &Vec<bool>, rows: u32, columns: u32, iter_count: u32) {
    stdout.write("\x1B[2J\x1B[2J\x1B[1;1H".as_bytes());
    stdout.write(("\u{25AC}".repeat(columns as usize) + "\n").as_bytes());
    let mut column: String = "".to_owned();
    let mut columns_string: String = "".to_owned();
    for r in 0..rows {
        for c in 0..columns {
            let b = map[(r * columns + c) as usize];
            let _ = match b {
                true => column.push_str("\u{2588}"),
                false => column.push_str(" ")
            };
            // column.push_str("\n");
            columns_string.push_str(column.as_str());
            column.clear();
        }
        stdout.write(columns_string.as_bytes());
    }
    stdout.write("\u{25AC}".repeat(columns as usize).as_bytes());
    stdout.write(iter_count.to_string().as_bytes());
    stdout.flush();
}
