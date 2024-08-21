// Conventional commit creator for terminal
use termion::cursor::DetectCursorPos;
use termion::{color, cursor, style};
mod structs;
use std::io::{stdin, stdout, Read, Write};
use termion::raw::IntoRawMode;

fn build_commit() {}
fn clear_terminal() {
    println!("{}", termion::clear::All);
}

fn commit_type<R: Read, W: Write>(term: &mut structs::terminal_interface::TerminalInterface<R, W>) {
    let questions = vec![
        "feat: ---------- [ ]",
        "fix:  ---------- [ ]",
        "chore:---------- [ ]",
        "docs: ---------- [ ]",
        "ref:  ---------- [ ]",
        "test: ---------- [ ]",
    ];
    term.clear_terminal();
    term.choice_question(questions, &color::Fg(color::Green));
    // term.write_line("Choose a commit type:", &color::Fg(color::Green));
    // term.write_line("1. feat: ---------- [ ]", &color::Fg(color::Green));
    // term.write_line("2. fix:  ---------- [ ]", &color::Fg(color::Green));
    // term.write_line("3. chore:---------- [ ]", &color::Fg(color::Green));
    // term.write_line("4. docs: ---------- [ ]", &color::Fg(color::Green));
    // term.write_line("5. ref:  ---------- [ ]", &color::Fg(color::Green));
    // term.write_line("6. test: ---------- [ ]", &color::Fg(color::Green));
    return;
}

fn main() {
    let stdout = stdout();
    let stdin = stdin();
    let stdin = stdin.lock();
    println!("{}Hello, world!{}", color::Fg(color::Red), style::Reset);
    println!("{}", termion::clear::All);
    println!("{}Hello, world!{}", color::Fg(color::Red), style::Reset);
    println!("Hello, world!");
    let stdout = stdout.into_raw_mode().unwrap();
    let mut term = structs::terminal_interface::TerminalInterface::new(stdin, stdout);
    commit_type(&mut term);
}
