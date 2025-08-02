use std::{
    io::{self, Stdout, Write, stdout},
    thread::sleep,
    time::Duration,
};

use crossterm::{ExecutableCommand, cursor, terminal};
use morphing_string::MorphingString;

// A poem by Refaat Alareer. https://ifimustdie.net/
const LINES: &[&str] = &[
    "If I must die,",
    "you must live",
    "to tell my story",
    "to sell my things",
    "to buy a piece of cloth",
    "and some strings,",
    "(make it white with a long tail)",
    "so that a child, somewhere in Gaza",
    "while looking heaven in the eye",
    "awaiting his dad who left in a blaze–",
    "and bid no one farewell",
    "not even to his flesh",
    "not even to himself–",
    "sees the kite, my kite you made, flying up above",
    "and thinks for a moment an angel is there",
    "bringing back love",
    "If I must die",
    "let it bring hope",
    "let it be a tale",
    "",
];

const MORPH_STEP_DURATION: Duration = Duration::from_millis(20);
const LINE_STEP_DURATION: Duration = Duration::from_secs(2);

fn main() {
    let mut out = stdout();
    let mut lines = LINES.iter().map(|line| line.to_string()).cycle();
    let mut line = MorphingString::new("".to_string());

    loop {
        let next_line = lines.next().expect("non-empty LINES cycled()d endlessly");
        line.set_target(next_line);
        clear_and_print(&line.value(), &mut out).unwrap();

        while !line.advance().is_complete() {
            clear_and_print(&line.value(), &mut out).unwrap();
            sleep(MORPH_STEP_DURATION);
        }

        sleep(LINE_STEP_DURATION);
    }
}

fn clear_and_print(line: &String, out: &mut Stdout) -> io::Result<()> {
    out.execute(cursor::MoveToColumn(0))?;
    out.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;

    write!(out, "{line}").expect("can write ");
    out.flush()
}
