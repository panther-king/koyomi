//! # 日付
//!
//! カレンダーとして必要な情報を持つ。
//!
//! - 年
//! - 月
//! - 日
//! - 曜日
//! - 和暦
//! - 祝祭日
use std::cmp::Ordering;
use std::fmt;

use chrono::{Datelike, NaiveDate, Weekday as ChronoWeekday};

use self::Weekday::*;
use super::{KoyomiError, KoyomiResult};
use crate::era;
use crate::holiday;

/// 曜日
///
/// 月曜を週の始まりとする。
/// [`chrono::Weekday`]から生成することもできる。
///
/// [chrono::Weekday]: https://docs.rs/chrono/0.4.0/chrono/enum.Weekday.html
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    /// 曜日の日本語表現を返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Weekday;
    ///
    /// let w = Weekday::Monday;
    /// assert_eq!(w.japanese(), '月');
    /// ```
    pub fn japanese(&self) -> char {
        match *self {
            Monday => '月',
            Tuesday => '火',
            Wednesday => '水',
            Thursday => '木',
            Friday => '金',
            Saturday => '土',
            Sunday => '日',
        }
    }
}

impl From<ChronoWeekday> for Weekday {
    /// [`chrono::Weekday`]から対応する曜日を生成する
    ///
    /// [`chrono::Weekday`]: https://docs.rs/chrono/0.4.0/chrono/enum.Weekday.html
    fn from(weekday: ChronoWeekday) -> Self {
        match weekday {
            ChronoWeekday::Mon => Monday,
            ChronoWeekday::Tue => Tuesday,
            ChronoWeekday::Wed => Wednesday,
            ChronoWeekday::Thu => Thursday,
            ChronoWeekday::Fri => Friday,
            ChronoWeekday::Sat => Saturday,
            ChronoWeekday::Sun => Sunday,
        }
    }
}

/// 日付
///
/// カレンダーのベースとなる構造体。
/// 文字列または(年, 月, 日)から生成する。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
    weekday: Weekday,
}

impl Date {
    /// 文字列からオブジェクトを生成する
    /// 文字列は `Y-m-d` か `Y/m/d` 形式のみ受け付ける
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Date;
    ///
    /// let date = Date::parse("2018-01-01");
    /// assert!(date.is_ok());
    ///
    /// let date = Date::parse("2018/01/01");
    /// assert!(date.is_ok());
    ///
    /// let date = Date::parse("2018-01-32");
    /// assert!(date.is_err());
    /// ```
    pub fn parse(fmt: &str) -> KoyomiResult<Self> {
        NaiveDate::parse_from_str(fmt, "%Y-%m-%d")
            .or(NaiveDate::parse_from_str(fmt, "%Y/%m/%d"))
            .map_err(|_| KoyomiError::InvalidFormat(fmt.into()))
            .map(|d| Date::from_chrono(d))
    }

    /// 年月日からオブジェクトを生成する
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Date;
    ///
    /// let date = Date::from_ymd(2018, 1, 1);
    /// assert!(date.is_ok());
    ///
    /// let date = Date::from_ymd(2018, 13, 1);
    /// assert!(date.is_err());
    /// ```
    pub fn from_ymd(year: i32, month: u32, day: u32) -> KoyomiResult<Self> {
        let ymd = format!("{:<4}-{:<02}-{:<02}", year, month, day);
        Date::parse(&ymd)
    }

    /// 「日」を返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Date;
    ///
    /// let date = Date::from_ymd(2018, 1, 31).unwrap();
    /// assert_eq!(date.day(), 31);
    /// ```
    pub fn day(&self) -> u32 {
        self.day
    }

    /// 「元号」を返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Date;
    ///
    /// let date = Date::from_ymd(2018, 1, 1).unwrap();
    /// assert_eq!(date.era().unwrap().name(), "平成");
    /// ```
    pub fn era(&self) -> Option<era::Era> {
        era::era(self)
    }

    /// 「祝祭日」を返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Date;
    ///
    /// let date = Date::from_ymd(2018, 1, 1).unwrap();
    /// assert_eq!(date.holiday().unwrap(), "元日");
    ///
    /// let date = Date::from_ymd(2018, 1, 2).unwrap();
    /// assert_eq!(date.holiday(), None);
    /// ```
    pub fn holiday(&self) -> Option<String> {
        holiday::holiday(self)
    }

    /// 「月」を返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Date;
    ///
    /// let date = Date::from_ymd(2018, 12, 1).unwrap();
    /// assert_eq!(date.month(), 12);
    /// ```
    pub fn month(&self) -> u32 {
        self.month
    }

    /// 日付間の期間が何日あるかを返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Date;
    ///
    /// let from = Date::from_ymd(2018, 1, 1).unwrap();
    /// let until = Date::from_ymd(2018, 12, 31).unwrap();
    /// assert_eq!(until.num_days(&from), 364);
    ///
    /// // うるう年
    /// let from = Date::from_ymd(2016, 1, 1).unwrap();
    /// let until = Date::from_ymd(2016, 12, 31).unwrap();
    /// assert_eq!(until.num_days(&from), 365);
    /// ```
    pub fn num_days(&self, date: &Date) -> i64 {
        let min = NaiveDate::from_ymd(self.year, self.month, self.day);
        let sub = NaiveDate::from_ymd(date.year(), date.month(), date.day());
        min.signed_duration_since(sub).num_days()
    }

    /// 日付の文字列表現を返す
    /// フォーマットは `Y-m-d` 形式となる
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Date;
    ///
    /// let date = Date::from_ymd(2018, 1, 1).unwrap();
    /// assert_eq!(date.to_string(), "2018-01-01");
    /// ```
    pub fn to_string(&self) -> String {
        format!("{:<4}-{:<02}-{:<02}", self.year, self.month, self.day)
    }

    /// 翌日の日付を返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Date;
    ///
    /// let date = Date::from_ymd(2018, 1, 1).unwrap();
    /// let tomorrow = date.tomorrow();
    /// assert_eq!(tomorrow.unwrap().to_string(), "2018-01-02");
    /// ```
    pub fn tomorrow(&self) -> KoyomiResult<Self> {
        NaiveDate::from_ymd(self.year, self.month, self.day)
            .succ_opt()
            .ok_or(KoyomiError::NoTomorrow(self.year, self.month, self.day))
            .map(|d| Date::from_chrono(d))
    }

    /// 「曜日」を返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::{Date, Weekday};
    ///
    /// let date = Date::from_ymd(2018, 1, 1).unwrap();
    /// assert_eq!(date.weekday(), &Weekday::Monday);
    /// ```
    pub fn weekday(&self) -> &Weekday {
        &self.weekday
    }

    /// 「年」を返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Date;
    ///
    /// let date = Date::from_ymd(2018, 1, 1).unwrap();
    /// assert_eq!(date.year(), 2018);
    /// ```
    pub fn year(&self) -> i32 {
        self.year
    }

    /// 前日の日付を返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Date;
    ///
    /// let date = Date::from_ymd(2018, 1, 1).unwrap();
    /// let yesterday = date.yesterday();
    /// assert_eq!(yesterday.unwrap().to_string(), "2017-12-31");
    /// ```
    pub fn yesterday(&self) -> KoyomiResult<Self> {
        NaiveDate::from_ymd(self.year, self.month, self.day)
            .pred_opt()
            .ok_or(KoyomiError::NoYesterday(self.year, self.month, self.day))
            .map(|d| Date::from_chrono(d))
    }

    /// `chrono`から日付オブジェクトを生成する
    fn from_chrono(date: NaiveDate) -> Self {
        Date {
            year: date.year(),
            month: date.month(),
            day: date.day(),
            weekday: Weekday::from(date.weekday()),
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Ord for Date {
    /// 日付オブジェクト同士を比較可能にする
    fn cmp(&self, other: &Date) -> Ordering {
        match self.year.cmp(&other.year) {
            Ordering::Equal => match self.month.cmp(&other.month) {
                Ordering::Equal => self.day.cmp(&other.day),
                ord => ord,
            },
            ord => ord,
        }
    }
}

impl PartialOrd for Date {
    /// 日付オブジェクト同士を比較可能にする
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::naive::{MAX_DATE, MIN_DATE};

    #[test]
    fn parse_hyphen_format() {
        assert!(Date::parse("2018-01-01").is_ok());
    }

    #[test]
    fn parse_slash_format() {
        assert!(Date::parse("2018/01/01").is_ok());
    }

    #[test]
    fn parse_invalid_format() {
        assert!(Date::parse("2018 01 01").is_err());
    }

    #[test]
    fn valid_ymd() {
        assert!(Date::from_ymd(2018, 1, 1).is_ok());
    }

    #[test]
    fn invalid_ymd() {
        assert!(Date::from_ymd(2018, 13, 1).is_err());
    }

    #[test]
    fn ymd_of_date() {
        let date = Date::parse("2018-12-24").unwrap();
        assert_eq!(date.year(), 2018);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 24);
    }

    #[test]
    fn era_of_date() {
        let date = Date::parse("2018-12-24").unwrap();
        assert_eq!(date.era().unwrap().name(), "平成");
    }

    #[test]
    fn holiday_of_date() {
        let date = Date::parse("2018-12-23").unwrap();
        assert_eq!(date.holiday().unwrap(), "天皇誕生日");
    }

    #[test]
    fn monday_of_weekday() {
        let weekday = Weekday::from(ChronoWeekday::Mon);
        assert_eq!(weekday, Monday);
        assert_eq!(weekday.japanese(), '月');
    }

    #[test]
    fn tuesday_of_weekday() {
        let weekday = Weekday::from(ChronoWeekday::Tue);
        assert_eq!(weekday, Tuesday);
        assert_eq!(weekday.japanese(), '火');
    }

    #[test]
    fn wednesday_of_weekday() {
        let weekday = Weekday::from(ChronoWeekday::Wed);
        assert_eq!(weekday, Wednesday);
        assert_eq!(weekday.japanese(), '水');
    }

    #[test]
    fn thursday_of_weekday() {
        let weekday = Weekday::from(ChronoWeekday::Thu);
        assert_eq!(weekday, Thursday);
        assert_eq!(weekday.japanese(), '木');
    }

    #[test]
    fn friday_of_weekday() {
        let weekday = Weekday::from(ChronoWeekday::Fri);
        assert_eq!(weekday, Friday);
        assert_eq!(weekday.japanese(), '金');
    }

    #[test]
    fn saturday_of_weekday() {
        let weekday = Weekday::from(ChronoWeekday::Sat);
        assert_eq!(weekday, Saturday);
        assert_eq!(weekday.japanese(), '土');
    }

    #[test]
    fn sunday_of_weekday() {
        let weekday = Weekday::from(ChronoWeekday::Sun);
        assert_eq!(weekday, Sunday);
        assert_eq!(weekday.japanese(), '日');
    }

    #[test]
    fn valid_tomorrow() {
        let date = Date::parse("2018-12-24").unwrap();
        assert!(date.tomorrow().is_ok());
    }

    #[test]
    fn invalid_tomorrow() {
        let date = Date::parse(&MAX_DATE.format("%Y-%m-%d").to_string()).unwrap();
        assert!(date.tomorrow().is_err());
    }

    #[test]
    fn valid_yesterday() {
        let date = Date::parse("2018-12-24").unwrap();
        assert!(date.yesterday().is_ok());
    }

    #[test]
    fn invalid_yesterday() {
        let date = Date::parse(&MIN_DATE.format("%Y-%m-%d").to_string()).unwrap();
        assert!(date.yesterday().is_err());
    }

    #[test]
    fn num_days() {
        let date = Date::parse("2018-04-01").unwrap();

        let sub = Date::parse("2018-03-01").unwrap();
        assert_eq!(date.num_days(&sub), 31);

        let sub = Date::parse("2018-05-01").unwrap();
        assert_eq!(date.num_days(&sub), -30);

        let sub = Date::parse("2018-04-01").unwrap();
        assert_eq!(date.num_days(&sub), 0);
    }

    #[test]
    fn date_to_string() {
        let format = "2018-01-01";
        let date = Date::parse(format).unwrap();
        assert_eq!(date.to_string(), format);
    }

    #[test]
    fn date_display() {
        let format = "2018-01-01";
        let date = Date::parse(format).unwrap();
        assert_eq!(format!("{}", date), format);
    }

    #[test]
    fn date_equal() {
        let d1 = Date::parse("2018-04-01").unwrap();
        let d2 = Date::parse("2018-04-01").unwrap();
        assert!(d1 == d2);
    }

    #[test]
    fn date_greater() {
        let d1 = Date::parse("2018-04-10").unwrap();
        let d2 = Date::parse("2017-04-10").unwrap();
        assert!(d1 > d2);

        let d2 = Date::parse("2018-03-20").unwrap();
        assert!(d1 > d2);

        let d2 = Date::parse("2018-04-01").unwrap();
        assert!(d1 > d2);
    }

    #[test]
    fn date_greater_than() {
        let d1 = Date::parse("2018-04-10").unwrap();
        let d2 = Date::parse("2018-04-10").unwrap();
        assert!(d1 >= d2);

        let d2 = Date::parse("2018-04-01").unwrap();
        assert!(d1 >= d2);
    }

    #[test]
    fn date_less() {
        let d1 = Date::parse("2018-04-10").unwrap();
        let d2 = Date::parse("2019-04-10").unwrap();
        assert!(d1 < d2);

        let d2 = Date::parse("2018-05-10").unwrap();
        assert!(d1 < d2);

        let d2 = Date::parse("2018-04-20").unwrap();
        assert!(d1 < d2);
    }

    #[test]
    fn date_less_than() {
        let d1 = Date::parse("2018-04-10").unwrap();
        let d2 = Date::parse("2018-04-10").unwrap();
        assert!(d1 <= d2);

        let d2 = Date::parse("2018-04-20").unwrap();
        assert!(d1 <= d2);
    }
}
