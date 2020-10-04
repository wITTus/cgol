use std::{thread, time};
use std::io::{stdout, Stdout, Write};

use clap::{App, Arg};

use crate::field::{Field};
use crate::pattern::Pattern;
use crate::gfx::*;

mod field;
mod pattern;
mod gfx;

fn main() {
    let matches = App::new("Conway's Game of Life").author("w177us")
        .about("Run with e.g. `cgol -c $COLUMNS -r $[ LINES-3 ] -i 30 -p patterns/glider.cells'")
        .arg(Arg::with_name("rows").short('r').about("Number of rows").takes_value(true))
        .arg(Arg::with_name("columns").short('c').about("Number of columns").takes_value(true))
        .arg(Arg::with_name("interval").short('t').about("Tick interval (in ms)").takes_value(true))
        .arg(Arg::with_name("pattern").short('p').about("Load pattern from file").takes_value(true))
        .arg(Arg::with_name("highres").short('x').about("Use high resolution"))
        .arg(Arg::with_name("mark").short('m').about("Mark pattern"))
        .arg(Arg::with_name("insert").short('i').about("Insert pattern"))
        .arg(Arg::with_name("mode").long("mode").possible_values(&["random", "empty"]))
        .get_matches();

    let rows = matches.value_of("rows").map(|v| v.parse::<usize>().unwrap()).unwrap_or(21);
    let columns = matches.value_of("columns").map(|v| v.parse::<usize>().unwrap()).unwrap_or(80);
    let interval = matches.value_of("interval").map(|v| v.parse::<u64>().unwrap()).unwrap_or(30);
    let pattern = matches.value_of("pattern").map(|p| Pattern::from_file(p).expect("Couldn't open .cells file"));
    let highres = matches.is_present("highres");
    let mark = matches.is_present("mark");
    let insert = matches.is_present("insert");
    let mode = matches.value_of("mode");

    let mut stdout = stdout();

    let mut field = match mode {
        Some("random") => Field::from_random(rows, columns),
        Some("empty") => Field::with_size(rows, columns),
        _ => Field::from_random(rows, columns),
    };

    if insert {
        if let Some(p) = pattern.clone() { field.insert(p) }
    }

    print(&mut stdout, gfx_cls());

    loop {
        let gfx = match highres {
            true => field.to_string_highres(),
            false => field.to_string()
        };

        print(&mut stdout, gfx.as_str());
        thread::sleep(time::Duration::from_millis(interval));
        field.next_iteration();

        if mark {
            if let Some(p) = pattern.as_ref() { field.mark_pattern(p) }
        }
    }
}

#[allow(unused_must_use)]
fn print(stdout: &mut Stdout, field: &str) {
    stdout.write(field.as_bytes());
    stdout.flush();
}
