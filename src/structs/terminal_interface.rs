use std::clone::Clone;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::str;
use termion::cursor;
use termion::raw::IntoRawMode;
use termion::{color::Color, cursor::DetectCursorPos};

#[derive(Debug)]
pub struct TerminalInterface<R, W: Write> {
    stdout: W,
    stdin: R,
    pub cursor_x: u16,
    pub cursor_y: u16,
}

impl<R: Read, W: Write> TerminalInterface<R, W> {
    pub fn new(stdin: R, mut stdout: W) -> TerminalInterface<R, W> {
        TerminalInterface {
            stdin,
            stdout,
            cursor_x: 1,
            cursor_y: 1,
        }
    }

    pub fn clear_terminal(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        self.cursor_x = 1;
        self.cursor_y = 1;
    }

    pub fn choice_question(&mut self, lines: Vec<&str>, color: &termion::color::Fg<impl Color>) {
        let mut variants_position: HashMap<(u16, u16), &str> = HashMap::new();
        for line in lines {
            let mut position = self.write_line(line, color);
            position.1 -= 2;
            variants_position.insert(position, line);
        }
        println!("{:?}", variants_position);
        let _first_position_cords = variants_position.keys().next().unwrap();
        self.set_cursor(_first_position_cords.0, _first_position_cords.1);
        while true {
            let direction = self.read_move();
            if self.move_coursor(direction) == 1 {
                break;
            }
        }
    }

    pub fn write_line<C: Color>(
        &mut self,
        line: &str,
        color: &termion::color::Fg<C>,
    ) -> (u16, u16) {
        (self.cursor_x, self.cursor_y) = self.stdout.cursor_pos().unwrap();
        write!(self.stdout, "{}{}{}\n", color, line, termion::style::Reset,).unwrap();
        self.stdout.flush().unwrap();
        (self.cursor_x, self.cursor_y) = self.stdout.cursor_pos().unwrap();
        let (cursor_x, cursor_y) = (self.cursor_x, self.cursor_y).clone();
        self.cursor_y += 1;
        self.cursor_x = 1;
        self.set_cursor(self.cursor_x, self.cursor_y);
        (cursor_x, cursor_y)
    }

    fn read_move(&mut self) -> char {
        let mut buf = [0];
        self.stdin.read(&mut buf).unwrap();
        buf[0] as char
    }

    fn move_coursor(&mut self, direction: char) -> u8 {
        match direction {
            'j' => {
                self.cursor_y += 1;
                self.set_cursor(self.cursor_x, self.cursor_y);
                return 0;
            }
            'k' => {
                if self.cursor_y == 0 {
                    return 1;
                }
                self.cursor_y -= 1;
                self.set_cursor(self.cursor_x, self.cursor_y);
                return 0;
            }
            'q' => {
                return 1;
            }
            _ => {
                return 0;
            }
        }
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> (u16, u16) {
        write!(self.stdout, "{}", cursor::Goto(x, y)).unwrap();
        self.stdout.flush().unwrap();
        self.cursor_x = x;
        self.cursor_y = y;
        (self.cursor_x, self.cursor_y)
    }
}
