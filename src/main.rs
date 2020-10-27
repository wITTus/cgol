extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::{thread, time};
use std::io::{stdout, Stdout, Write};

use clap::{App, Arg};

use crate::field::Field;
use crate::game::Game;
use crate::rule::AutomataRule;
use crate::term::*;

mod game;
mod field;
mod term;
mod rule;

// 24 - 1 (Iterations) - 2 (Horizontal Line)
const TERM_DEFAULT_ROWS: usize = 24 - 1 - 2;
const TERM_DEFAULT_COLUMNS: usize = 80;

fn main() {
    let matches = App::new("Conway's Game of Life").author("w177us")
        .about("Run with e.g. `cgol -c $COLUMNS -r $[ LINES-3 ] -t 30'")
        .arg(Arg::with_name("rows").short('r').about("Number of rows").takes_value(true))
        .arg(Arg::with_name("columns").short('c').about("Number of columns").takes_value(true))
        .arg(Arg::with_name("interval").short('t').about("Tick interval (in ms)").takes_value(true))
        .arg(Arg::with_name("highres").short('x').about("Use high resolution"))
        .arg(Arg::with_name("mark").short('m').takes_value(true).about("Mark pattern"))
        .arg(Arg::with_name("insert").short('i').takes_value(true).about("Insert pattern"))
        .arg(Arg::with_name("init").long("init").possible_values(&["empty", "random", "gauss"]))
        .arg(Arg::with_name("rule").long("rule").takes_value(true).about("Cellular automaton rule, e.g. B36/S23 for highlife."))
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
    let mark = matches.value_of("mark").map(|p| Field::from_file(p).expect("Couldn't open file"));
    let insert = matches.value_of("insert").map(|p| Field::from_file(p).expect("Couldn't open file"));
    let init = matches.value_of("init");
    let rule = matches.value_of("rule").map(AutomataRule::from).unwrap_or_else(AutomataRule::cgol);

    let mut stdout = stdout();

    let mut field = match init {
        Some("empty") => Field::with_size(rows, columns),
        Some("random") => Field::from_random(rows, columns),
        Some("gauss") => Field::from_normal_distribution(rows, columns),
        _ => Field::from_random(rows, columns)
    };

    if let Some(pattern) = insert { field.insert(pattern, 0, 0) }

    let mut game = Game::new(field, rule);

    print(&mut stdout, gfx_cls());

    loop {
        let gfx = match highres {
            true => game.to_string_highres(),
            false => game.to_string()
        };

        print(&mut stdout, gfx.as_str());
        thread::sleep(time::Duration::from_millis(interval));
        game.next_iteration();

        if let Some(pattern) = mark.as_ref() { game.mark_pattern(pattern) }
    }
}

#[allow(unused_must_use)]
fn print(stdout: &mut Stdout, field: &str) {
    stdout.write(field.as_bytes());
    stdout.flush();
}
