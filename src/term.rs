use std::process::Command;

pub fn colormap_gb(n: u32) -> String {
    match n {
        0 => String::from("\x1B[38;5;34m"),
        1 => String::from("\x1B[38;5;35m"),
        2 => String::from("\x1B[38;5;36m"),
        3 => String::from("\x1B[38;5;37m"),
        4 => String::from("\x1B[38;5;38m"),
        5 => String::from("\x1B[38;5;39m"),
        _ => String::from("\x1B[38;5;21m")
    }
}

pub const fn gfx_cls() -> &'static str {
    "\x1B[2J\x1B[1;1H"
}

pub const fn gfx_pos1() -> &'static str {
    "\x1B[1;1H"
}

pub fn gfx_cell(alive: bool, color: String) -> String {
    let s = "\u{2588}";

    match alive {
        true => color + s,
        false => String::from(" ")
    }
}

pub fn gfx_cell_highres(ul: bool, ur: bool, bl: bool, br: bool, color: String) -> String {
    let ws = " ".to_string();

    let symbol = match (ul, ur, bl, br) {
        (false, false, false, false) => ws.as_str(),
        (false, false, false, true) => "\u{2597}",
        (false, false, true, false) => "\u{2596}",
        (false, false, true, true) => "\u{2584}",
        (false, true, false, false) => "\u{259D}",
        (false, true, false, true) => "\u{2590}",
        (false, true, true, false) => "\u{259E}",
        (false, true, true, true) => "\u{259F}",
        (true, false, false, false) => "\u{2598}",
        (true, false, false, true) => "\u{259A}",
        (true, false, true, false) => "\u{258C}",
        (true, false, true, true) => "\u{2599}",
        (true, true, false, false) => "\u{2580}",
        (true, true, false, true) => "\u{259C}",
        (true, true, true, false) => "\u{259B}",
        (true, true, true, true) => "\u{2588}",
    };

    match symbol {
        " " => ws,
        s => color + s
    }
}

pub fn gfx_hline(columns: usize) -> String {
    "\x1B[38;5;15m".to_string() + "\u{25AC}".repeat(columns).as_str()
}

pub fn gfx_hline_highres(columns: usize) -> String {
    "\x1B[38;5;15m".to_string() + "\u{25AC}".repeat(columns / 2).as_str()
}

pub(crate) fn call(cmd: &str, arg: &str) -> Option<String> {
    let output = Command::new(cmd).arg(arg).output().ok()?;
    let string = String::from_utf8_lossy(&output.stdout).trim_end().to_string();
    Some(string)
}