extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::{thread, time};
use std::io::{stdout, Stdout, Write};

use clap::{App, Arg};

use crate::game::Game;
use crate::field::Field;
use crate::term::*;

mod game;
mod field;
mod term;

const TERM_DEFAULT_ROWS: usize = 21;
// 24 - 1 (Iterations) - 2 (Horizontal Line)
const TERM_DEFAULT_COLUMNS: usize = 80;

fn main() {
    let matches = App::new("Conway's Game of Life").author("w177us")
        .about("Run with e.g. `cgol -c $COLUMNS -r $[ LINES-3 ] -t 30'")
        .arg(Arg::with_name("rows").short('r').about("Number of rows").takes_value(true))
        .arg(Arg::with_name("columns").short('c').about("Number of columns").takes_value(true))
        .arg(Arg::with_name("interval").short('t').about("Tick interval (in ms)").takes_value(true))
        .arg(Arg::with_name("pattern").short('p').about("Load pattern from file").takes_value(true))
        .arg(Arg::with_name("highres").short('x').about("Use high resolution"))
        .arg(Arg::with_name("mark").short('m').about("Mark pattern"))
        .arg(Arg::with_name("insert").short('i').about("Insert pattern"))
        .arg(Arg::with_name("mode").long("mode").possible_values(&["random", "empty"]))
        .get_matches();

    let highres = matches.is_present("highres");

    let rows = matches.value_of("rows").map(|s| s.to_string())
        .or_else(|| call("tput", "lines"))
        .and_then(|s| s.parse::<usize>().ok())
        .map(|i| (i - 3) * if highres { 2 } else { 1 })
        .unwrap_or(TERM_DEFAULT_ROWS);

    let columns = matches.value_of("columns").map(|s| s.to_string())
        .or_else(|| call("tput", "cols"))
        .and_then(|s| s.parse::<usize>().ok())
        .map(|i| i * (if highres { 2 } else { 1 }))
        .unwrap_or(TERM_DEFAULT_COLUMNS);

    let interval = matches.value_of("interval").map(|v| v.parse::<u64>().unwrap()).unwrap_or(30);
    let pattern = matches.value_of("pattern").map(|p| Field::from_file(p).expect("Couldn't open file"));
    let mark = matches.is_present("mark");
    let insert = matches.is_present("insert");
    let mode = matches.value_of("mode");

    let mut stdout = stdout();

    let mut field = match mode {
        Some("empty") => Field::with_size(rows, columns),
        Some("random") => Field::from_random(rows, columns),
        _ => Field::from_random(rows, columns)
    };

    if insert {
        if let Some(p) = pattern.clone() { field.insert(p) }
    }

    let mut game = Game::new(field);

    print(&mut stdout, gfx_cls());

    loop {
        let gfx = match highres {
            true => game.to_string_highres(),
            false => game.to_string()
        };

        print(&mut stdout, gfx.as_str());
        thread::sleep(time::Duration::from_millis(interval));
        game.next_iteration();

        if mark {
            if let Some(p) = pattern.as_ref() { game.mark_pattern(p) }
        }
    }
}

#[allow(unused_must_use)]
fn print(stdout: &mut Stdout, field: &str) {
    stdout.write(field.as_bytes());
    stdout.flush();
}
