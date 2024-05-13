use std::io::Write;

use crossterm::{cursor, execute, queue, terminal, style, event};
use crossterm::event::{Event, KeyCode};
use crossterm::style::Stylize;

pub struct ConsoleView {

}

impl ConsoleView {
    pub fn update(&self) -> crate::Result<()> {
        let (width, height) = terminal::size()?;
        let mut stdout = std::io::stdout();
        queue!(stdout,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo((width / 2) - 10, 1), style::Print("マスター　マインド".yellow()),
        )?;
        for i in 1..=10 {
            queue!(stdout,
                cursor::MoveTo((width / 2) - 15, 13 - i), style::Print(format!("{:>2}: {}|", i, "|    ".repeat(4))),
            )?;
        }
        queue!(stdout,
            cursor::MoveTo((width / 2) - 8, 14), style::Print("1    2    3    4"),
            cursor::MoveTo((width / 2) - 15 - 24, 17), style::Print("ピンの位置を選んでください (1-4)"),
            cursor::MoveTo((width / 2) - 15 - 24, 19), style::Print("ピンを選んでください    "),
                style::Print(" q:"), style::Print("▲".red()),
                style::Print(" w:"), style::Print("▲".blue()),
                style::Print(" e:"), style::Print("▲".green()),
                style::Print(" r:"), style::Print("▲".yellow()),
                style::Print(" t:"), style::Print("▲".with(style::Color::Rgb { r:255, g:165, b:0 })),
                style::Print(" y:"), style::Print("▲".with(style::Color::Rgb { r:247, g:155, b:185 })),
            cursor::MoveTo(width - 10, height - 2), style::Print("終了: ESC  "),
        )?;
        queue!(stdout,
            cursor::MoveTo(width - 10, height - 2), style::Print("終了: ESC  "),
            cursor::MoveTo(0, height - 1),
        )?;
        stdout.flush()?;
        
        Ok(())
    }

    pub fn wait_input(&self) -> crate::Result<()> {
        loop {
            // Blocks until an `Event` is available

            let event = event::read()?;
            //println!("{:?}", event);
            match event {
                Event::Key(key) if key.kind == event::KeyEventKind::Release => {
                    match key.code {
                        event::KeyCode::Esc => break,
                        event::KeyCode::Char(ch) => {
                            if '1' <= ch && ch <= '6' {

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

}