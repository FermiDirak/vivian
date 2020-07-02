use std::io::Write;

use termion::{clear, color, cursor};

use super::container::Container;

pub fn clear_screen<W: Write>(stdout: &mut W) {
    write!(
        stdout,
        "{}{}{}",
        clear::All,
        cursor::Goto(1, 1),
        cursor::Hide,
    )
    .unwrap();
}

pub fn fill_area<W: Write>(stdout: &mut W, container: &Container) {
    let Container {
        x,
        y,
        width,
        height,
    } = *container;
    let spaces = " ".repeat(width.into());

    for y_curr in y..y + height {
        write!(
            stdout,
            "{}{}{}{}",
            cursor::Goto(x, y_curr),
            color::Bg(color::Red),
            spaces,
            color::Bg(color::Reset),
        )
        .unwrap();
    }
}

pub fn write_text<W: Write>(stdout: &mut W, text: &String, pos: (u16, u16)) {
    let (x, y) = pos;
    write!(
        stdout,
        "{}{}{}{}",
        cursor::Goto(x, y),
        color::Bg(color::Red),
        text,
        color::Bg(color::Reset),
    )
    .unwrap();
}
