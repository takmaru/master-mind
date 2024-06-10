use std::fmt;
use std::io::Write;

use crossterm::{cursor, event, execute, queue, style, terminal };
use crossterm::event::{Event, KeyCode};
use crossterm::style::Stylize;

use crate::{Pin, Hint, History};

impl fmt::Display for Pin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "▲".with(self.color))
    }
}

impl fmt::Display for Hint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blow => write!(f, "□"),
            Self::Hit => write!(f, "■"),
            Self::None => write!(f, "  "),
        }
    }
}

struct Position { x: u16, y: u16 }

struct AnswerView {
    answer: Vec<Option<Pin>>,
}

impl<'a> fmt::Display for AnswerView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|")?;
        self.answer.iter()
            .for_each(|pin|
                match pin {
                    Some(pin) => write!(f, " {} |", pin).unwrap(),
                    None => write!(f, "    |").unwrap(),
                });
        Ok(())
    }
}

pub struct ConsoleView {
    width: u16,
    height: u16,
    answer_count: u32,
    try_count: u32,
    pinnum_group: SelectGroup<u32>,
    pins_group: SelectGroup<Pin>,
}

impl ConsoleView {

    pub fn new(pins: &[&Pin], answer_count: u32, try_count: u32) -> Self {
        let (width, height) = terminal::size().unwrap();

        let keys = [ '1', '2', '3', '4', '5', '6', '7', '8', '9', '0' ];
        let pinnum_group = SelectGroup::new(
            Position { x: (width / 2) - (answer_count as u16 * 5 / 2), y: 2 + try_count as u16 + 2 },
            keys.into_iter().zip(1..=answer_count)
            .map(|(key, item)| KeyItem { key, item: item.clone() })
            .collect());

        let keys = [ 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p' ];
        let pins_group = SelectGroup::new(
            Position { x: (width / 2) - (((pins.len() * 4) + pins.len() - 1) / 2) as u16, y: pinnum_group.position.y + 2 },
            keys.into_iter().zip(pins.iter())//.map(|p| *p))
                        .map(|(key, item)| KeyItem { key, item: (*item).clone() } )
                        .collect());

        execute!(std::io::stdout(), terminal::EnterAlternateScreen).unwrap();

        Self { width, height, try_count, answer_count, pinnum_group, pins_group }
    }

    pub fn update(&self) -> crate::Result<()> {
        let mut stdout = std::io::stdout();
        queue!(stdout,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo((self.width / 2) - 10, 1), style::Print("マスター　マインド".yellow()),
        )?;
        let x = (self.width / 2) - (((5 * self.answer_count) + 1) / 2) as u16 - 1;
        for i in 1..=self.try_count {
            queue!(stdout,
                cursor::MoveTo(x - 4, 2 + i as u16),
                style::Print(format!("{:>2}: {}", i, AnswerView { answer: vec![None; self.answer_count as usize] })),
            )?;
        }
        queue!(stdout,
            cursor::MoveTo(self.pinnum_group.position.x - 34, self.pinnum_group.position.y), style::Print("ピンの位置を選択してください"),
            cursor::MoveTo(self.pins_group.position.x - 30, self.pins_group.position.y), style::Print("ピンを選択してください"),
            cursor::MoveTo(self.width - 10, self.height - 2), style::Print("終了: ESC  "),
            cursor::MoveTo(0, self.height - 1),
        )?;
        stdout.flush()?;
        
        Ok(())
    }

    pub fn wait_input(&mut self, histories: &[History]) -> crate::Result<Vec<Pin>> {

        let x = (self.width / 2) - (((5 * self.answer_count) + 1) / 2) as u16 - 1;
        let y = 2 + self.try_count as u16;
        for (i, history) in histories.iter().enumerate() {
            view_history(x, y - i as u16, history);
        }

        let mut answer = AnswerWindow {
            position: Position { x, y: y - histories.len() as u16 },
            answer: AnswerView { answer: vec![None; self.answer_count as usize] },
        };
        
        self.pinnum_group.update_line();
        self.pins_group.update_line();
        
        loop {
            {
                let mut stdout = std::io::stdout();
                queue!(stdout,
                    cursor::MoveTo(answer.position.x + (answer.answer.answer.len() * 5 + 1) as u16 + 2, answer.position.y),
                    terminal::Clear(terminal::ClearType::UntilNewLine))?;
                if answer.answer.answer.iter().all(|a| a.is_some()) {
                    queue!(stdout, style::Print("決定: ENT"))?;
                }
                stdout.flush()?;
            }

            let event = event::read()?;
            //println!("{:?}", event);
            match event {
                Event::Key(key) if key.kind == event::KeyEventKind::Release => {
                    match key.code {
                        KeyCode::Esc => return Err(Box::new(crate::Error::EndOfEscape)),
                        KeyCode::Char(ch) => {
                            if let Some(num) = self.pinnum_group.select(Some(ch)) {
                                if let Some(pin) = self.pins_group.select_value() {
                                    answer.input_pin((num - 1) as usize, pin);
                                    self.pinnum_group.select(None);
                                    self.pins_group.select(None);
                                    execute!(std::io::stdout(),
                                        cursor::MoveTo(4, self.height - 4), terminal::Clear(terminal::ClearType::CurrentLine),
                                        style::Print(format!("ピン: {} を 位置: {} にセット", pin, num)))?;
                                } else {
                                    execute!(std::io::stdout(),
                                        cursor::MoveTo(4, self.height - 4), terminal::Clear(terminal::ClearType::CurrentLine),
                                        style::Print(format!("ピンの位置: {} を選択", num)))?;
                                }
                            } else if let Some(pin) = self.pins_group.select(Some(ch)) {
                                if let Some(num) = self.pinnum_group.select_value() {
                                    answer.input_pin((num - 1) as usize, pin);
                                    self.pinnum_group.select(None);
                                    self.pins_group.select(None);
                                    execute!(std::io::stdout(),
                                        cursor::MoveTo(4, self.height - 4), terminal::Clear(terminal::ClearType::CurrentLine),
                                        style::Print(format!("ピン: {} を 位置: {} にセット", pin, num)))?;
                                } else {
                                    execute!(std::io::stdout(),
                                        cursor::MoveTo(4, self.height - 4), terminal::Clear(terminal::ClearType::CurrentLine),
                                        style::Print(format!("ピン: {} を選択", pin)))?;
                                }
                            } else {
                                execute!(std::io::stdout(),
                                    cursor::MoveTo(4, self.height - 4), terminal::Clear(terminal::ClearType::CurrentLine), style::Print(format!("'{}' キー じゃないよ", ch)))?;
                            }
                        },
                        KeyCode::Enter => {
                            if answer.answer.answer.iter().all(|a| a.is_some()) {
                                execute!(std::io::stdout(),
                                    cursor::MoveTo(answer.position.x + (answer.answer.answer.len() * 5 + 1) as u16 + 2 + 9 + 2, answer.position.y),
                                    style::Print("本当にいいですか？ (y/n)"))?;
                                loop {
                                    let event = event::read()?;
                                    match event {
                                        Event::Key(key) if key.kind == event::KeyEventKind::Release => {
                                            match key.code {
                                                KeyCode::Char(ch) if ch == 'y' => {
                                                    return Ok(answer.answer.answer.iter().map(|a| a.unwrap()).collect());
                                                },
                                                KeyCode::Char(ch) if ch == 'n' => break,
                                                KeyCode::Esc => return Err(Box::new(crate::Error::EndOfEscape)),
                                                _ => (),
                                            }
                                        },
                                        _ => (),
                                    }
                                }
                            }
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        }

        Ok(Vec::new())
    }
}

struct HistoryPins<'a>(&'a Vec<Pin>);
impl<'a> fmt::Display for HistoryPins<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|")?;
        self.0.iter().for_each(|pin| write!(f, " {} |", pin).unwrap());
        Ok(())
    }
}

struct HistoryHints<'a>(&'a Vec<Hint>);
impl<'a> fmt::Display for HistoryHints<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().for_each(|hiht| write!(f, "{}", hiht).unwrap());
        Ok(())
    }
}

fn view_history(x: u16, y: u16, history: &History) {
    execute!(std::io::stdout(),
        cursor::MoveTo(x, y), terminal::Clear(terminal::ClearType::UntilNewLine),
        style::Print(format!("{} {}", HistoryPins(&history.pins), HistoryHints(&history.hints)))
    ).unwrap();
}

impl Drop for ConsoleView {
    fn drop(&mut self) {
        execute!(std::io::stdout(), terminal::LeaveAlternateScreen).unwrap();
    }
}

struct KeyItem<T: fmt::Display> {
    key: char,
    item: T,
}

impl fmt::Display for KeyItem<Pin> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.key, self.item)
    }
}

impl fmt::Display for KeyItem<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  {} ", self.item)
    }
}

struct AnswerWindow {
    position: Position,
    answer: AnswerView,
}

impl AnswerWindow {

    fn update(&self) {
        let mut stdout = std::io::stdout();
        queue!(stdout,
            cursor::MoveTo(self.position.x, self.position.y),
            style::Print(format!("{}", self.answer))).unwrap();
        stdout.flush();
    }

    fn input_pin(&mut self, pos: usize, pin: Pin) {
        self.answer.answer = self.answer.answer.iter().enumerate()
            .map(|(i, a)|
                if i == pos {
                    Some(pin)
                } else if let Some(p) = a {
                    if *p == pin { None }
                    else { Some(*p) }
                } else { None }).collect();
        self.update();
    }
}

struct SelectGroup<T>
    where T: fmt::Display {
    position: Position,
    values: Vec<KeyItem<T>>,
    selecting: Option<char>,
}

impl<T> SelectGroup<T>
    where KeyItem<T>: std::fmt::Display, T: std::fmt::Display + Clone {
    fn new(position: Position, values: Vec<KeyItem<T>>) -> Self {
        Self { position, values, selecting: None }
    }

    fn update_line(&self) {
        let mut stdout = std::io::stdout();
        queue!(stdout, cursor::MoveTo(self.position.x, self.position.y)).unwrap();
        for value in &self.values {
            match self.selecting {
                Some(k) if k == value.key =>
                    queue!(stdout,
                        style::SetBackgroundColor(style::Color::DarkGrey),
                        style::Print(format!("{}", value)),
                        style::ResetColor).unwrap(),
                _ => queue!(stdout, style::Print(format!("{}", value))).unwrap(),
            }
            queue!(stdout, cursor::MoveRight(1)).unwrap();
        };
        stdout.flush();
    }

    fn select(&mut self, s: Option<char>) -> Option<T> {
        if self.selecting == s {
            return self.select_value();
        } else {
            if let Some(key) = s {
                let find = self.values.iter().find_map(|v| if v.key == key { Some(v.item.clone()) } else { None } );
                if find.is_some() {
                    self.selecting = s;
                    self.update_line();
                }
                return find;
            } else {
                self.selecting = None;
                self.update_line();
                return None;
            }
        }
    }

    fn select_value(&self) -> Option<T> {
        match self.selecting {
            Some(key) => return self.values.iter().find_map(|v| if v.key == key { Some(v.item.clone()) } else { None } ),
            None => return None,
        }
    }
}
