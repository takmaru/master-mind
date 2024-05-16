use std::io::Write;
use std::collections::HashMap;

use crossterm::{cursor, terminal, style, event, execute, queue, };
use crossterm::event::{Event, KeyCode};
use crossterm::style::Stylize;

use crate::Pin;

struct Position { x: u16, y: u16 }
pub struct ConsoleView {
    width: u16,
    height: u16,
    pin_keys: Vec<PinKey>,
    try_count: u32,
    answer_count: u32,
    pin_num_pos: Position,
    pins_pos: Position,
}

struct PinKey {
    pin: Pin,
    key: char,
}

impl ConsoleView {

    pub fn new(pins: &[&Pin], answer_count: u32, try_count: u32) -> Self {
        let (width, height) = terminal::size().unwrap();
        let keys = [ 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', ];
        let pin_keys: Vec<PinKey> = pins.iter().enumerate()
                                        .map(|(i, p)|
                                                PinKey { pin: **p, key:keys[i]}).collect();
        let pin_num_pos = Position { x: (width / 2) - (answer_count as u16 * 4 / 2) - 1, y: 2 + try_count as u16 + 2 };
        let pins_pos = Position { x: (width / 2) - (((pin_keys.len() * 4) + pin_keys.len() - 1) / 2) as u16, y: pin_num_pos.y + 2 };

        execute!(std::io::stdout(), terminal::EnterAlternateScreen).unwrap();
        Self { width, height, pin_keys, try_count, answer_count, pins_pos, pin_num_pos }
    }

    pub fn update(&self) -> crate::Result<()> {
        let mut stdout = std::io::stdout();
        queue!(stdout,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo((self.width / 2) - 10, 1), style::Print("マスター　マインド".yellow()),
        )?;
        for i in 1..=10 {
            queue!(stdout,
                cursor::MoveTo((self.width / 2) - 15, 13 - i), style::Print(format!("{:>2}: {}|", i, "|    ".repeat(self.answer_count as usize))),
            )?;
        }
        queue!(stdout,
            cursor::MoveTo(self.width - 10, self.height - 2), style::Print("終了: ESC  "),
            cursor::MoveTo(0, self.height - 1),
        )?;
        stdout.flush()?;
        
        Ok(())
    }

    pub fn wait_input(&self) -> crate::Result<()> {

        self.print_nums(None);
        self.print_pins(None);

        loop {
            let event = event::read()?;
            //println!("{:?}", event);
            match event {
                Event::Key(key) if key.kind == event::KeyEventKind::Release => {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Char(ch) => {
                            if '1' <= ch && ch <= '4' {
                                self.print_nums(Some(ch.to_digit(10).unwrap()));
                                execute!(std::io::stdout(),
                                    cursor::MoveTo(4, self.height - 4), terminal::Clear(terminal::ClearType::CurrentLine),
                                    style::Print(format!("ピンの位置: {} を選択", ch)))?;
                            } else if let Some(pk) = self.pin_keys.iter().find(|pk| pk.key == ch) {
                                self.print_pins(Some(ch));
                                execute!(std::io::stdout(),
                                    cursor::MoveTo(4, self.height - 4), terminal::Clear(terminal::ClearType::CurrentLine),
                                    style::Print("ピン: "),
                                    style::Print("▲".with(pk.pin.color)), 
                                    style::Print(" を選択"))?;
                            } else {
                                execute!(std::io::stdout(),
                                    cursor::MoveTo(4, self.height - 4), terminal::Clear(terminal::ClearType::CurrentLine), style::Print(format!("'{}' キー じゃないよ", ch)))?;
                            }
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        }

        Ok(())
    }

    fn print_nums(&self, select: Option<u32>) {
        let mut stdout = std::io::stdout();
        queue!(stdout,
            cursor::MoveTo(self.pin_num_pos.x - 33, self.pin_num_pos.y),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("ピンの位置を選択してください"),
            cursor::MoveTo(self.pin_num_pos.x, self.pin_num_pos.y),
        ).unwrap();
        for num in 1..=self.answer_count {
            match select {
                Some(s) if s == num =>
                    queue!(stdout,
                        style::SetBackgroundColor(style::Color::DarkGrey),
                        style::Print(format!(" {} ", num)),
                        style::ResetColor,
                        cursor::MoveRight(1)).unwrap(),
                _ => queue!(stdout, style::Print(format!(" {} ", num)), cursor::MoveRight(1)).unwrap(),
            }
            queue!(stdout, cursor::MoveRight(1)).unwrap();
        }
        stdout.flush();
    }

    fn print_pins(&self, select: Option<char>) {
        let mut stdout = std::io::stdout();
        queue!(stdout,
            cursor::MoveTo(self.pins_pos.x - 28, self.pins_pos.y),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("ピンを選択してください"),
            cursor::MoveTo(self.pins_pos.x, self.pins_pos.y)).unwrap();
        for (i, pk) in self.pin_keys.iter().enumerate() {
            match select {
                Some(s) if s == pk.key =>
                    queue!(stdout,
                        style::SetBackgroundColor(style::Color::DarkGrey),
                        style::Print(format!("{}:", pk.key)), style::Print("▲".with(pk.pin.color)),
                        style::ResetColor).unwrap(),
                _ => queue!(stdout, style::Print(format!("{}:", pk.key)), style::Print("▲".with(pk.pin.color))).unwrap(),
            }
            queue!(stdout, cursor::MoveRight(1)).unwrap();
        };
        stdout.flush();
    }
}

impl Drop for ConsoleView {
    fn drop(&mut self) {
        execute!(std::io::stdout(), terminal::LeaveAlternateScreen).unwrap();
    }
}