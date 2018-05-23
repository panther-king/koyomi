extern crate chrono;

mod calendar;
mod date;
mod era;
mod holiday;

pub use calendar::Calendar;
pub use calendar::CalendarBuilder;
pub use calendar::is_leap;
pub use calendar::num_days;

pub use date::Date;
pub use date::Weekday;

pub use era::{era, Era};

pub use holiday::holiday;

/// # クレート単位の`Result`
///
/// 失敗時は `KoyomiError` を返す
pub type KoyomiResult<T> = Result<T, KoyomiError>;

/// # クレート単位のエラーバリアント
#[derive(Debug)]
pub enum KoyomiError {
    /// 妥当な日付ではない
    InvalidFormat(String),
    /// カレンダーの期間指定が妥当ではない
    InvalidTerm(Date, Date),
    /// カレンダーを生成するための指定が不足している
    NotEnough,
    /// 指定日の翌日は存在しない
    NoTomorrow(i32, u32, u32),
    /// 指定日の前日は存在しない
    NoYesterday(i32, u32, u32),
}
