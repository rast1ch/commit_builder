// Conventional commit creator for terminal
use termion::{color, style};
mod structs;

fn main() {
    println!("{}Hello, world!{}", color::Fg(color::Red), style::Reset);
    println!("{}", termion::clear::All);
    println!("{}Hello, world!{}", color::Fg(color::Red), style::Reset);
    println!("Hello, world!");
}
