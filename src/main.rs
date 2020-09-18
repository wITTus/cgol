extern crate termion;

use std::{thread, time};
use std::io::{stdout, Stdout, Write};

use rand::Rng;

const ROWS: i32 = 24;
const COLUMNS: i32 = 80;

fn main() {
    let mut stdout = stdout();
    let mut rng = rand::thread_rng();
    let mut field = (0..COLUMNS * ROWS).map(|_| rng.gen_bool(0.5)).collect::<Vec<bool>>();

    loop {
        print(&mut stdout, &field);
        thread::sleep(time::Duration::from_millis(50));
        field = apply_rules(field);
    }
}

fn apply_rules(field: Vec<bool>) -> Vec<bool> {
    field.iter().enumerate()
        .map(|(i, alive)| match neighbours(&field, i as i32) {
            2 => true & alive,
            3 => true,
            _ => false
        }).collect()
}

fn neighbours(m: &Vec<bool>, index: i32) -> usize {
    [
        index - COLUMNS - 1, index - COLUMNS, index - COLUMNS + 1,
        index - 1, /*                      */ index + 1,
        index + COLUMNS - 1, index + COLUMNS, index + COLUMNS + 1
    ]
        .iter()
        .map(|&idx| match idx {
            i if i < 0 => false,
            i if i >= COLUMNS * ROWS => false,
            i => *m.get(i as usize).expect("Lookup failed")
        })
        .filter(|i| { matches!(i, true) })
        .count()
}

#[allow(unused_must_use)]
fn print(stdout: &mut Stdout, map: &Vec<bool>) {
    stdout.write("\x1B[2J\x1B[2J\x1B[1;1H".as_bytes());
    stdout.write("================================================================================\n".as_bytes());
    for r in 0..ROWS {
        for c in 0..COLUMNS {
            let b = map[(r * COLUMNS + c) as usize];
            let c = match b {
                true => "\u{2588}",
                false => " "
            };
            stdout.write(c.as_bytes());
        }
        stdout.write("\n".as_bytes());
    }
    stdout.write("================================================================================\n".as_bytes());
    stdout.flush();
}
