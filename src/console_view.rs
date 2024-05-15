use std::io::Write;
use std::collections::HashMap;

use crossterm::{cursor, terminal, style, event, execute, queue, };
use crossterm::event::{Event, KeyCode};
use crossterm::style::Stylize;

use crate::Pin;

pub struct ConsoleView {
    width: u16,
    height: u16,
    pin_keys: Vec<PinKey>,
    pins_x: u16,
}

struct PinKey {
    pin: Pin,
    key: char,
}

impl ConsoleView {

    pub fn new(pins: &[&Pin]) -> Self {
        let (width, height) = terminal::size().unwrap();
        let keys = [ 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', ];
        let pin_keys: Vec<PinKey> = pins.iter().enumerate()
                                        .map(|(i, p)|
                                                PinKey { pin: **p, key:keys[i]}).collect();
        let pins_x = (width / 2) - (((pin_keys.len() * 4) + pin_keys.len() - 1) / 2) as u16;
        execute!(std::io::stdout(), terminal::EnterAlternateScreen).unwrap();
        Self { width, height, pin_keys, pins_x }
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
                cursor::MoveTo((self.width / 2) - 15, 13 - i), style::Print(format!("{:>2}: {}|", i, "|    ".repeat(4))),
            )?;
        }
        queue!(stdout,
            cursor::MoveTo((self.width / 2) - 15 - 24, 14), style::Print("ピンの位置を選んでください"),
            cursor::MoveTo((self.width / 2) - 8, 14), style::Print("1    2    3    4"),
            cursor::MoveTo(self.width - 10, self.height - 2), style::Print("終了: ESC  "),
            cursor::MoveTo(0, self.height - 1),
        )?;
        stdout.flush()?;
        
        Ok(())
    }

    pub fn wait_input(&self) -> crate::Result<()> {

        self.print_pins(16, None);

        loop {
            let event = event::read()?;
            //println!("{:?}", event);
            match event {
                Event::Key(key) if key.kind == event::KeyEventKind::Release => {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Char(ch) => {
                            if '1' <= ch && ch <= '4' {
                                execute!(std::io::stdout(),
                                    cursor::MoveTo(4, self.height - 4), terminal::Clear(terminal::ClearType::CurrentLine),
                                    style::Print(format!("ピンの位置: {} を選択", ch)))?;
                            } else if let Some(pk) = self.pin_keys.iter().find(|pk| pk.key == ch) {
                                self.print_pins(16, Some(ch));
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

    fn print_pins(&self, row: u16, select: Option<char>) {
        let mut stdout = std::io::stdout();
        queue!(stdout,
            cursor::MoveTo(self.pins_x - 26, row),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("ピンを選択してください"),
            cursor::MoveRight(4)).unwrap();
        for (i, pk) in self.pin_keys.iter().enumerate() {
            let pin_x = self.pins_x + i as u16 * 5;
            queue!(stdout, cursor::MoveTo(self.pins_x + i as u16 * 5, row)).unwrap();
            match select {
                Some(s) if s == pk.key =>
                    queue!(stdout,
                        style::SetBackgroundColor(style::Color::DarkGrey),
                        style::Print(format!("{}:", pk.key)), style::Print("▲".with(pk.pin.color)),
                        style::ResetColor).unwrap(),
                _ => queue!(stdout, style::Print(format!("{}:", pk.key)), style::Print("▲".with(pk.pin.color))).unwrap(),
            }
        };
        stdout.flush();
    }
}

impl Drop for ConsoleView {
    fn drop(&mut self) {
        execute!(std::io::stdout(), terminal::LeaveAlternateScreen).unwrap();
    }
}