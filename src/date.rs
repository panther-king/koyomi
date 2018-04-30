//! 日付
use std::cmp::Ordering;

use chrono::{Datelike, NaiveDate, Weekday as ChronoWeekday};

use era;
use holiday;
use self::KoyomiError::*;
use self::Weekday::*;

#[derive(Debug)]
pub enum KoyomiError {
    InvalidDate(String),
    NoTomorrow(i32, u32, u32),
    NoYesterday(i32, u32, u32),
}

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
    weekday: Weekday,
}

impl Date {
    pub fn parse(fmt: &str) -> Result<Self, KoyomiError> {
        NaiveDate::parse_from_str(fmt, "%Y-%m-%d")
            .or(NaiveDate::parse_from_str(fmt, "%Y/%m/%d"))
            .map_err(|_| InvalidDate(fmt.into()))
            .map(|d| Date::from_chrono(d))
    }

    pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<Self, KoyomiError> {
        let ymd = format!("{:<4}-{:<02}-{:<02}", year, month, day);
        Date::parse(&ymd)
    }

    pub fn day(&self) -> u32 {
        self.day
    }

    pub fn era(&self) -> Option<era::Era> {
        era::era(self)
    }

    pub fn holiday(&self) -> Option<String> {
        holiday::holiday(self)
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn tomorrow(&self) -> Result<Self, KoyomiError> {
        NaiveDate::from_ymd(self.year, self.month, self.day)
            .succ_opt()
            .ok_or(NoTomorrow(self.year, self.month, self.day))
            .map(|d| Date::from_chrono(d))
    }

    pub fn weekday(&self) -> &Weekday {
        &self.weekday
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn yesterday(&self) -> Result<Self, KoyomiError> {
        NaiveDate::from_ymd(self.year, self.month, self.day)
            .pred_opt()
            .ok_or(NoYesterday(self.year, self.month, self.day))
            .map(|d| Date::from_chrono(d))
    }

    fn from_chrono(date: NaiveDate) -> Self {
        Date {
            year: date.year(),
            month: date.month(),
            day: date.day(),
            weekday: Weekday::from(date.weekday()),
        }
    }
}

impl Ord for Date {
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
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[cfg(test)]
mod tests {
    use chrono::naive::{MAX_DATE, MIN_DATE};
    use super::*;

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
