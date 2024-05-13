use std::error;
use std::fmt;
use std::collections::HashSet;

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::thread_rng;
use rand::seq::IteratorRandom;

mod console_view;
use console_view::ConsoleView;

#[derive(EnumIter, Clone, PartialEq, Eq, Hash, Debug)]
enum Pin { Blue, Green, Orange, Pink, Red, Yellow }

#[derive(Debug)]
pub enum Error {
    NoAnswer,   // ゲームの回答を生成できなかった
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoAnswer => write!(f, "答えを生成できなかった"),
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
    pins: Vec<Pin>,
    answer_count: u32,
    try_count: u32,
}

impl Rule {
    fn answer(&self) -> Result<Answer> {
        let answer = self.pins.clone().into_iter().permutations(self.answer_count as usize).choose(&mut thread_rng())
            .ok_or(Error::NoAnswer)?;
        Ok(Answer { answer })
    }
}

pub fn start() -> Result<()> {

    // ルール、答えを準備
    let rule = Rule {pins: Pin::iter().collect(), answer_count: 4, try_count: 10 };
    let answer = rule.answer()?;
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
        let try_pins = vec![ Pin::Red, ];
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
        let answer = Answer { answer: vec![ Pin::Red, Pin::Blue, Pin::Green, Pin::Yellow ] };

        // all Hit
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Blue, Pin::Green, Pin::Yellow ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::Hit, Hint::Hit ]);

        // 2 Hit, 2 Blow
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Red, Pin::Green, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Blue, Pin::Red, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Blue, Pin::Green, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Green, Pin::Blue, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Yellow, Pin::Green, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Blue, Pin::Yellow, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::Hit ]);

        // 1 Hit, 3 Blow
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Yellow, Pin::Blue, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Green, Pin::Yellow, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Blue, Pin::Red, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Blue, Pin::Yellow, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Red, Pin::Green, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Yellow, Pin::Green, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Red, Pin::Blue, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Green, Pin::Red, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Hit ]);

        // all Blow
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Red, Pin::Yellow, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Green, Pin::Yellow, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Yellow, Pin::Red, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Red, Pin::Yellow, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Yellow, Pin::Red, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Yellow, Pin::Blue, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Red, Pin::Blue, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Green, Pin::Red, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Green, Pin::Blue, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::Blow ]);

        // 3 Hit, 1 None
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Blue, Pin::Green, Pin::Yellow ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Orange, Pin::Green, Pin::Yellow ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Blue, Pin::Pink, Pin::Yellow ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Blue, Pin::Green, Pin::Orange ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::Hit, Hint::None ]);

        // 2 Hit, 1 Blow, 1 None
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Blue, Pin::Orange, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Blue, Pin::Yellow, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Pink, Pin::Green, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Yellow, Pin::Green, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Orange, Pin::Blue, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Green, Pin::Pink, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Blue, Pin::Green, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Blue, Pin::Green, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Blue, Pin::Red, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Blue, Pin::Orange, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Red, Pin::Green, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Pink, Pin::Green, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::Hit, Hint::None ]);

        // 1 Hit, 2 Blow, 1 None
        //  Hit=Red
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Green, Pin::Blue, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Pink, Pin::Blue, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Green, Pin::Orange, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Yellow, Pin::Blue, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Yellow, Pin::Orange, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Pink, Pin::Yellow, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Green, Pin::Yellow, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Yellow, Pin::Orange, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Pink, Pin::Yellow, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        //  Hit=Blue
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Blue, Pin::Red, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Blue, Pin::Red, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Blue, Pin::Orange, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Blue, Pin::Red, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Blue, Pin::Orange, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Blue, Pin::Yellow, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Blue, Pin::Yellow, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Blue, Pin::Pink, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Blue, Pin::Yellow, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        //  Hit=Green
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Red, Pin::Green, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Yellow, Pin::Green, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Yellow, Pin::Green, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Orange, Pin::Green, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Pink, Pin::Green, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Orange, Pin::Green, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Red, Pin::Green, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Red, Pin::Green, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Yellow, Pin::Green, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        //  Hit=Yellow
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Red, Pin::Blue, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Green, Pin::Red, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Green, Pin::Blue, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Pink, Pin::Red, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Orange, Pin::Red, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Pink, Pin::Blue, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Red, Pin::Orange, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Red, Pin::Pink, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Green, Pin::Orange, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Hit, Hint::None ]);

        // 0 Hit, 3 Blow, 1 None(1st)
        //      RBG
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Red, Pin::Blue, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Green, Pin::Red, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Green, Pin::Blue, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RBY
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Red, Pin::Yellow, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Yellow, Pin::Red, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Yellow, Pin::Blue, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RGY
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Red, Pin::Yellow, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Yellow, Pin::Red, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Green, Pin::Yellow, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      BGY
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Yellow, Pin::Blue, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Green, Pin::Yellow, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        // 0 Hit, 3 Blow, 1 None(2nd)
        //      RBG
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Orange, Pin::Red, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Pink, Pin::Red, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Orange, Pin::Blue, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RBY
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Pink, Pin::Red, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Orange, Pin::Yellow, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Pink, Pin::Blue, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RGY
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Pink, Pin::Red, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Orange, Pin::Yellow, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      BGY
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Orange, Pin::Yellow, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Pink, Pin::Blue, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Orange, Pin::Yellow, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        // 0 Hit, 3 Blow, 1 None(3rd)
        //      RBG
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Red, Pin::Pink, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Red, Pin::Orange, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Green, Pin::Pink, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RBY
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Red, Pin::Orange, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Yellow, Pin::Pink, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RGY
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Red, Pin::Pink, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Yellow, Pin::Orange, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Green, Pin::Pink, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      BGY
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Yellow, Pin::Orange, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Yellow, Pin::Pink, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Green, Pin::Orange, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        // 0 Hit, 3 Blow, 1 None(4th)
        //      RBG
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Red, Pin::Blue, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Green, Pin::Red, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RBY
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Red, Pin::Yellow, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Red, Pin::Blue, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Yellow, Pin::Red, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      RGY
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Red, Pin::Yellow, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Yellow, Pin::Red, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Green, Pin::Red, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        //      BGY
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Green, Pin::Yellow, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Yellow, Pin::Blue, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Green, Pin::Blue, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::Blow, Hint::None ]);

        // 2 Hit, 2 None
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Blue, Pin::Orange, Pin::Pink ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Pink, Pin::Green, Pin::Orange ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Orange, Pin::Pink, Pin::Yellow ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Blue, Pin::Green, Pin::Pink ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Blue, Pin::Orange, Pin::Yellow ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Pink, Pin::Green, Pin::Yellow ]).unwrap(),  vec![ Hint::Hit, Hint::Hit, Hint::None, Hint::None ]);

        // 1 Hit, 1 Blow, 2 None
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Orange, Pin::Blue, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Pink, Pin::Orange, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Green, Pin::Pink, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Pink, Pin::Orange, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Yellow, Pin::Pink, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Red, Pin::Orange, Pin::Yellow, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Blue, Pin::Red, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Blue, Pin::Orange, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Blue, Pin::Orange, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Blue, Pin::Pink, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Blue, Pin::Orange, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Blue, Pin::Yellow, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Red, Pin::Green, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Orange, Pin::Green, Pin::Red ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Orange, Pin::Green, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Orange, Pin::Green, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Orange, Pin::Green, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Yellow, Pin::Green, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Red, Pin::Pink, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Orange, Pin::Red, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Orange, Pin::Pink, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Orange, Pin::Blue, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Orange, Pin::Pink, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Green, Pin::Orange, Pin::Yellow ]).unwrap(),  vec![ Hint::Blow, Hint::Hit, Hint::None, Hint::None ]);

        // 0 Hit, 2 Blow, 2 None
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Pink, Pin::Red, Pin::Blue ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Pink, Pin::Red, Pin::Orange, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Orange, Pin::Yellow, Pin::Red, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Blue, Pin::Pink, Pin::Orange, Pin::Green ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Yellow, Pin::Orange, Pin::Blue, Pin::Pink ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::None, Hint::None ]);
        assert_eq!(answer.judge(&vec![ Pin::Green, Pin::Yellow, Pin::Pink, Pin::Orange ]).unwrap(),  vec![ Hint::Blow, Hint::Blow, Hint::None, Hint::None ]);

        // less
        assert!(answer.judge(&vec![ Pin::Red ]).is_none());
        assert!(answer.judge(&vec![ Pin::Red, Pin::Green ]).is_none());
        assert!(answer.judge(&vec![ Pin::Red, Pin::Green, Pin::Blue ]).is_none());

        // duplicate
        assert!(answer.judge(&vec![ Pin::Red, Pin::Green, Pin::Blue, Pin::Red ]).is_none());
        assert!(answer.judge(&vec![ Pin::Red, Pin::Green, Pin::Green, Pin::Yellow ]).is_none());
        assert!(answer.judge(&vec![ Pin::Red, Pin::Blue, Pin::Blue, Pin::Yellow ]).is_none());
        assert!(answer.judge(&vec![ Pin::Yellow, Pin::Green, Pin::Blue, Pin::Yellow ]).is_none());
        assert!(answer.judge(&vec![ Pin::Red, Pin::Pink, Pin::Orange, Pin::Pink ]).is_none());
    }
}
