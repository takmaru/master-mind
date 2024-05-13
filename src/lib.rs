use std::error;
use std::fmt;
use std::collections::HashSet;
use std::pin;

use itertools::Itertools;
//use strum::IntoEnumIterator;
//use strum_macros::EnumIter;
use rand::thread_rng;
use rand::seq::IteratorRandom;
use crossterm::style::Color;

mod console_view;
use console_view::ConsoleView;

//#[derive(EnumIter, Clone, PartialEq, Eq, Hash, Debug)]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Pin {
    color: Color,
}

impl Pin {
    fn red() -> Pin { Pin { color: Color::Red } }
    fn green() -> Pin { Pin { color: Color::Green } }
    fn blue() -> Pin { Pin { color: Color::Blue } }
    fn yellow() -> Pin { Pin { color: Color::Yellow } }
    fn pink() -> Pin { Pin { color: Color::Rgb { r:247, g:155, b:185 } } }
    fn orange() -> Pin { Pin { color: Color::Rgb { r:255, g:165, b:0 } } }
}

#[derive(Debug)]
pub enum Error {
    AnswerNew { pins_len: usize, count:usize },   // 答えを生成できなかった
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::AnswerNew { pins_len, count } => write!(f, "Answer::new() error. pins.len:{}, count:{}", pins_len, count),
        }
    }
}

impl error::Error for Error {}

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(PartialEq, PartialOrd, Debug)]
enum Hint { Blow, Hit, None, }

#[derive(Debug)]
struct Answer {
    answer: Vec<Pin>,
}

impl Answer {

    fn new(pins: &HashSet<Pin>, count: usize) -> Result<Answer> {
        let answer = pins.clone().into_iter().permutations(count).choose(&mut thread_rng())
            .ok_or(Error::AnswerNew { pins_len: pins.len(), count} )?;
        Ok(Answer { answer })
    }

    fn judge(&self, pins: &[Pin]) -> Option<Vec<Hint>> {
        // 数のチェック
        if pins.len() != self.answer.len() { return None; }

        let mut duplicate_check = HashSet::new();
        let mut hints = vec![];
        for (idx, pin) in pins.iter().enumerate() {
            // 重複チェック
            if !duplicate_check.insert(pin) { return None; }
            for (a_idx, a_pin) in self.answer.iter().enumerate() {
                if pin == a_pin {
                    // 答え(answer)に同じ色があり、位置もあっていれば Hit、位置が違えば Blow
                    hints.push(if idx == a_idx { Hint::Hit } else { Hint::Blow });
                    break;
                }
            }
            // 答え(answer)に同じ色がなければ None
            if hints.len() == idx { hints.push(Hint::None) };
        }

        // ヒントをソートして返す(Blow->Hit->None)
        hints.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Some(hints)
    }
}
struct Rule {
    pins: HashSet<Pin>,
    answer_count: u32,
    try_count: u32,
}

pub fn start() -> Result<()> {

    // ルール
    let pins = HashSet::from([ Pin::red(), Pin::green(), Pin::blue(), Pin::yellow(), Pin::pink(), Pin::orange() ]);
    let rule = Rule { pins, answer_count: 4, try_count: 10 };
    // 答え
    let answer = Answer::new(&rule.pins, rule.answer_count as usize)?;
    println!("answer: {:?}", answer);

    let mut view = ConsoleView{};
    view.update()?;
    view.wait_input();
    return Ok(());

    // 最大回数まで
    let mut try_count = 1;
    while try_count < rule.try_count {

        // 現在の状況を表示する
        // 入力を待つ
        // 入力を判定する
        
        // 入力
        let try_pins = vec![ ];
        if let hints = answer.judge(&try_pins) {
            // 次の入力
        } else {
            // 結果なし（回答のピンが足りない or 重複がある）
            continue;
        }
        match answer.judge(&try_pins) {
            // 結果あり
            Some(hints) => (),
            // 結果なし（回答のピンが足りない or 重複がある）
            None => continue,
        };
        try_count += 1;
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer_judge() {
        let answer = Answer { answer: vec![ Pin::red(), Pin::blue(), Pin::green(), Pin::yellow() ] };

        // all Hit
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::blue(), Pin::green(), Pin::yellow() ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::Hit, Hint::Hit ]);

        // 2 Hit, 2 Blow
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::red(), Pin::green(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::blue(), Pin::red(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::blue(), Pin::green(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::green(), Pin::blue(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::yellow(), Pin::green(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::blue(), Pin::yellow(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::Hit ]);

        // 1 Hit, 3 Blow
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::yellow(), Pin::blue(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::green(), Pin::yellow(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::blue(), Pin::red(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::blue(), Pin::yellow(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::red(), Pin::green(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::yellow(), Pin::green(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::red(), Pin::blue(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::green(), Pin::red(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);

        // all Blow
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::red(), Pin::yellow(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::green(), Pin::yellow(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::yellow(), Pin::red(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::red(), Pin::yellow(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::yellow(), Pin::red(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::yellow(), Pin::blue(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::red(), Pin::blue(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::green(), Pin::red(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::green(), Pin::blue(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);

        // 3 Hit, 1 None
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::blue(), Pin::green(), Pin::yellow() ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::orange(), Pin::green(), Pin::yellow() ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::blue(), Pin::pink(), Pin::yellow() ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::blue(), Pin::green(), Pin::orange() ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::Hit, Hint::None ]);

        // 2 Hit, 1 Blow, 1 None
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::blue(), Pin::orange(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::blue(), Pin::yellow(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::pink(), Pin::green(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::yellow(), Pin::green(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::orange(), Pin::blue(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::green(), Pin::pink(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::blue(), Pin::green(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::blue(), Pin::green(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::blue(), Pin::red(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::blue(), Pin::orange(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::red(), Pin::green(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::pink(), Pin::green(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);

        // 1 Hit, 2 Blow, 1 None
        //  Hit=Red
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::green(), Pin::blue(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::pink(), Pin::blue(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::green(), Pin::orange(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::yellow(), Pin::blue(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::yellow(), Pin::orange(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::pink(), Pin::yellow(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::green(), Pin::yellow(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::yellow(), Pin::orange(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::pink(), Pin::yellow(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        //  Hit=Blue
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::blue(), Pin::red(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::blue(), Pin::red(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::blue(), Pin::orange(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::blue(), Pin::red(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::blue(), Pin::orange(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::blue(), Pin::yellow(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::blue(), Pin::yellow(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::blue(), Pin::pink(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::blue(), Pin::yellow(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        //  Hit=Green
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::red(), Pin::green(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::yellow(), Pin::green(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::yellow(), Pin::green(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::orange(), Pin::green(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::pink(), Pin::green(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::orange(), Pin::green(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::red(), Pin::green(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::red(), Pin::green(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::yellow(), Pin::green(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        //  Hit=Yellow
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::red(), Pin::blue(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::green(), Pin::red(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::green(), Pin::blue(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::pink(), Pin::red(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::orange(), Pin::red(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::pink(), Pin::blue(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::red(), Pin::orange(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::red(), Pin::pink(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::green(), Pin::orange(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);

        // 0 Hit, 3 Blow, 1 None(1st)
        //      RBG
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::red(), Pin::blue(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::green(), Pin::red(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::green(), Pin::blue(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RBY
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::red(), Pin::yellow(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::yellow(), Pin::red(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::yellow(), Pin::blue(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RGY
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::red(), Pin::yellow(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::yellow(), Pin::red(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::green(), Pin::yellow(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      BGY
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::yellow(), Pin::blue(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::green(), Pin::yellow(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        // 0 Hit, 3 Blow, 1 None(2nd)
        //      RBG
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::orange(), Pin::red(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::pink(), Pin::red(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::orange(), Pin::blue(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RBY
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::pink(), Pin::red(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::orange(), Pin::yellow(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::pink(), Pin::blue(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RGY
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::pink(), Pin::red(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::orange(), Pin::yellow(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      BGY
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::orange(), Pin::yellow(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::pink(), Pin::blue(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::orange(), Pin::yellow(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        // 0 Hit, 3 Blow, 1 None(3rd)
        //      RBG
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::red(), Pin::pink(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::red(), Pin::orange(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::green(), Pin::pink(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RBY
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::red(), Pin::orange(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::yellow(), Pin::pink(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RGY
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::red(), Pin::pink(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::yellow(), Pin::orange(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::green(), Pin::pink(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      BGY
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::yellow(), Pin::orange(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::yellow(), Pin::pink(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::green(), Pin::orange(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        // 0 Hit, 3 Blow, 1 None(4th)
        //      RBG
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::red(), Pin::blue(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::green(), Pin::red(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RBY
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::red(), Pin::yellow(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::red(), Pin::blue(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::yellow(), Pin::red(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RGY
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::red(), Pin::yellow(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::yellow(), Pin::red(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::green(), Pin::red(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      BGY
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::green(), Pin::yellow(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::yellow(), Pin::blue(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::green(), Pin::blue(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);

        // 2 Hit, 2 None
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::blue(), Pin::orange(), Pin::pink() ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::pink(), Pin::green(), Pin::orange() ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::orange(), Pin::pink(), Pin::yellow() ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::blue(), Pin::green(), Pin::pink() ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::blue(), Pin::orange(), Pin::yellow() ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::pink(), Pin::green(), Pin::yellow() ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::None, Hint::None ]);

        // 1 Hit, 1 Blow, 2 None
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::orange(), Pin::blue(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::pink(), Pin::orange(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::green(), Pin::pink(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::pink(), Pin::orange(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::yellow(), Pin::pink(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::red(), Pin::orange(), Pin::yellow(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::blue(), Pin::red(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::blue(), Pin::orange(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::blue(), Pin::orange(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::blue(), Pin::pink(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::blue(), Pin::orange(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::blue(), Pin::yellow(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::red(), Pin::green(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::orange(), Pin::green(), Pin::red() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::orange(), Pin::green(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::orange(), Pin::green(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::orange(), Pin::green(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::yellow(), Pin::green(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::red(), Pin::pink(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::orange(), Pin::red(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::orange(), Pin::pink(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::orange(), Pin::blue(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::orange(), Pin::pink(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::green(), Pin::orange(), Pin::yellow() ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);

        // 0 Hit, 2 Blow, 2 None
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::pink(), Pin::red(), Pin::blue() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::pink(), Pin::red(), Pin::orange(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::orange(), Pin::yellow(), Pin::red(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::blue(), Pin::pink(), Pin::orange(), Pin::green() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::yellow(), Pin::orange(), Pin::blue(), Pin::pink() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::green(), Pin::yellow(), Pin::pink(), Pin::orange() ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::None, Hint::None ]);

        // less
        assert!(answer.judge(&vec![ Pin::red() ]).is_none());
        assert!(answer.judge(&vec![ Pin::red(), Pin::green() ]).is_none());
        assert!(answer.judge(&vec![ Pin::red(), Pin::green(), Pin::blue() ]).is_none());

        // duplicate
        assert!(answer.judge(&vec![ Pin::red(), Pin::green(), Pin::blue(), Pin::red() ]).is_none());
        assert!(answer.judge(&vec![ Pin::red(), Pin::green(), Pin::green(), Pin::yellow() ]).is_none());
        assert!(answer.judge(&vec![ Pin::red(), Pin::blue(), Pin::blue(), Pin::yellow() ]).is_none());
        assert!(answer.judge(&vec![ Pin::yellow(), Pin::green(), Pin::blue(), Pin::yellow() ]).is_none());
        assert!(answer.judge(&vec![ Pin::red(), Pin::pink(), Pin::orange(), Pin::pink() ]).is_none());
    }
}
