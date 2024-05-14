use std::io::Write;
use std::collections::HashMap;

use crossterm::{cursor, execute, queue, terminal, style, event};
use crossterm::event::{Event, KeyCode};
use crossterm::style::Stylize;

use crate::Pin;

pub struct ConsoleView {
    width: u16,
    height: u16,
    pin_keys: [char; 10],
}

impl ConsoleView {

    pub fn new() -> Self {
        let (width, height) = terminal::size().unwrap();
        let pin_keys = [ 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', ];
        Self { width, height, pin_keys }
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
            cursor::MoveTo((self.width / 2) - 8, 14), style::Print("1    2    3    4"),
            cursor::MoveTo((self.width / 2) - 15 - 24, 17), style::Print("ピンの位置を選んでください (1-4)"),
            cursor::MoveTo((self.width / 2) - 15 - 24, 19), style::Print("ピンを選んでください    "),
            cursor::MoveTo(self.width - 10, self.height - 2), style::Print("終了: ESC  "),
            cursor::MoveTo(0, self.height - 1),
        )?;
        stdout.flush()?;
        
        Ok(())
    }

    pub fn wait_input(&self, pins: &[&Pin] ) -> crate::Result<()> {

        let mut pin_map = HashMap::new();
        {
            let mut stdout = std::io::stdout();
            queue!(stdout, cursor::MoveTo((self.width / 2) - 15, 19), terminal::Clear(terminal::ClearType::UntilNewLine))?;
            let mut key = self.pin_keys.iter();
            for pin in pins {
                let key_char = key.next().unwrap();
                pin_map.insert(key_char, *pin);
                queue!(stdout, style::Print(format!(" {}:", key_char)), style::Print("▲".with(pin.color)))?;

            }
            queue!(stdout, cursor::MoveTo(0, self.height - 1))?;
            stdout.flush()?;
        }
        let pin_map = pin_map;

        loop {
            let event = event::read()?;
            //println!("{:?}", event);
            match event {
                Event::Key(key) if key.kind == event::KeyEventKind::Release => {
                    match key.code {
                        event::KeyCode::Esc => break,
                        event::KeyCode::Char(ch) if '1' <= ch && ch <= '4' => {
                                execute!(std::io::stdout(),
                                    cursor::MoveTo(4, self.height - 4), terminal::Clear(terminal::ClearType::CurrentLine),
                                    style::Print(format!("ピンの位置: {} を選択", ch)))?;
                        },
                        event::KeyCode::Char(ch) if pin_map.contains_key(&ch) => {
                                execute!(std::io::stdout(),
                                    cursor::MoveTo(4, self.height - 4), terminal::Clear(terminal::ClearType::CurrentLine),
                                    style::Print("ピン: "),
                                    style::Print("▲".with(pin_map.get(&ch).unwrap().color)), 
                                    style::Print(" を選択"))?;
                        },
                        event::KeyCode::Char(ch) => {
                            execute!(std::io::stdout(),
                                cursor::MoveTo(4, self.height - 4), terminal::Clear(terminal::ClearType::CurrentLine), style::Print(format!("{:?} キー じゃないよ", ch)))?;
                        }
                        _ => (),
                    }
                },
                _ => (),
            }
        }

        Ok(())
    }

}