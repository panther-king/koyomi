//! # カレンダー
//!
//! 指定期間の日付を持つカレンダーとユーティリティ関数
use super::{KoyomiError, KoyomiResult};
use date::Date;

/// 指定年月が何日まであるかを返す
///
/// # Examples
///
/// ```rust
/// use koyomi::num_days;
///
/// assert_eq!(num_days(2018, 1), 31);
///
/// // うるう年
/// assert_eq!(num_days(2016, 2), 29);
/// ```
pub fn num_days(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap(year) {
            29
        } else {
            28
        },
        _ => unreachable!(format!("{} is invalid month!", month)),
    }
}

/// うるう年かどうかを判定する
///
/// # Examples
///
/// ```rust
/// use koyomi::is_leap;
///
/// assert_eq!(is_leap(2016), true);
/// assert_eq!(is_leap(2018), false);
/// ```
pub fn is_leap(year: i32) -> bool {
    // @see https://www.nao.ac.jp/faq/a0306.html
    if year % 4 != 0 {
        false
    } else if year % 100 == 0 && year % 400 != 0 {
        false
    } else {
        true
    }
}

/// カレンダー
///
/// 指定期間の日付を生成することができる
#[derive(Debug)]
pub struct Calendar {
    from: Date,
    until: Date,
}

impl Calendar {
    /// カレンダーオブジェクトを生成する
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::{Calendar, Date};
    ///
    /// let from = Date::from_ymd(2018, 1, 1).unwrap();
    /// let until = Date::from_ymd(2018, 12, 31).unwrap();
    /// let cal = Calendar::new(from, until);
    /// assert!(cal.is_ok());
    ///
    /// let from = Date::from_ymd(2018, 12, 31).unwrap();
    /// let until = Date::from_ymd(2018, 1, 1).unwrap();
    /// let cal = Calendar::new(from, until);
    /// assert!(cal.is_err());
    /// ```
    pub fn new(from: Date, until: Date) -> KoyomiResult<Self> {
        if until.num_days(&from) <= 0 {
            Err(KoyomiError::InvalidTerm(from, until))
        } else {
            Ok(Calendar { from, until })
        }
    }

    /// カレンダーを生成するためのビルダーを返す
    pub fn build<'a>() -> CalendarBuilder<'a> {
        CalendarBuilder {
            from: None,
            single: None,
            until: None,
        }
    }

    /// 開始日の文字列表現を返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::{Calendar, Date};
    ///
    /// let from = Date::from_ymd(2018, 1, 1).unwrap();
    /// let until = Date::from_ymd(2018, 12, 31).unwrap();
    /// let cal = Calendar::new(from, until).unwrap();
    /// assert_eq!(cal.from(), "2018-01-01");
    /// ```
    pub fn from(&self) -> String {
        self.from.to_string()
    }

    pub fn make(&self) -> Vec<Date> {
        let days = self.until.num_days(&self.from) + 1;
        let mut cal = Vec::with_capacity(days as usize);
        let mut date = self.from.tomorrow();

        cal.push(self.from.clone());

        loop {
            if let Ok(d) = date {
                if d > self.until {
                    break;
                }
                date = d.tomorrow();
                cal.push(d);
            }
        }

        cal
    }

    /// 終了日の文字列表現を返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::{Calendar, Date};
    ///
    /// let from = Date::from_ymd(2018, 1, 1).unwrap();
    /// let until = Date::from_ymd(2018, 12, 31).unwrap();
    /// let cal = Calendar::new(from, until).unwrap();
    /// assert_eq!(cal.until(), "2018-12-31");
    /// ```
    pub fn until(&self) -> String {
        self.until.to_string()
    }
}

/// カレンダー用ビルダー
///
/// カレンダーの範囲指定には複数のユースケースがあるので、
/// それぞれに応じたカレンダーを生成するためのビルダー
///
/// 1. 特定年月のカレンダー
/// 2. 特定年のカレンダー
/// 3. 期間を年月で指定したカレンダー
/// 4. 期間を年で指定したカレンダー
#[derive(Debug)]
pub struct CalendarBuilder<'a> {
    from: Option<&'a str>,
    single: Option<&'a str>,
    until: Option<&'a str>,
}

impl<'a> CalendarBuilder<'a> {
    /// カレンダーオブジェクトを生成する
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Calendar;
    ///
    /// let builder = Calendar::build().single("2018").finalize();
    /// assert!(builder.is_ok());
    ///
    /// let builder = Calendar::build().from("2018").until("2019").finalize();
    /// assert!(builder.is_ok());
    ///
    /// let builder = Calendar::build().from("January").finalize();
    /// assert!(builder.is_err());
    /// ```
    pub fn finalize(&self) -> KoyomiResult<Calendar> {
        if let Some(single) = self.single {
            self.single_calendar(single)
        } else {
            let from = self.date_from()?;
            let until = self.date_until()?;
            Calendar::new(from, until)
        }
    }

    /// 期間の始まりを指定する
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Calendar;
    ///
    /// let mut builder = Calendar::build();
    /// builder.from("2018");
    /// ```
    pub fn from(mut self, from: &'a str) -> Self {
        self.from = Some(from);
        self
    }

    /// 単一年または単一年月を指定する
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Calendar;
    ///
    /// let mut builder = Calendar::build();
    /// builder.single("2018-01");
    /// ```
    pub fn single(mut self, single: &'a str) -> Self {
        self.single = Some(single);
        self
    }

    /// 期間の終わりを指定する
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::Calendar;
    ///
    /// let mut builder = Calendar::build();
    /// builder.until("2018");
    /// ```
    pub fn until(mut self, until: &'a str) -> Self {
        self.until = Some(until);
        self
    }

    /// カレンダーの開始日を導出する
    fn date_from(&self) -> KoyomiResult<Date> {
        match self.from {
            None => Err(KoyomiError::NotEnough),
            Some(f) => match f.split("-").count() {
                1 => Date::parse(&format!("{}-01-01", f)),
                2 => Date::parse(&format!("{}-01", f)),
                _ => Err(KoyomiError::InvalidFormat(f.into())),
            },
        }
    }

    /// カレンダーの終了日を導出する
    fn date_until(&self) -> KoyomiResult<Date> {
        match self.until {
            None => Err(KoyomiError::NotEnough),
            Some(u) => match u.split("-").count() {
                1 => Date::parse(&format!("{}-12-31", u)),
                2 => self.end_of_month(&format!("{}-01", u)),
                _ => Err(KoyomiError::InvalidFormat(u.into())),
            },
        }
    }

    /// 指定月の末日を導出する
    fn end_of_month(&self, fmt: &str) -> KoyomiResult<Date> {
        let first = Date::parse(fmt)?;
        let (y, m) = (first.year(), first.month());
        Date::from_ymd(y, m, num_days(y, m))
    }

    /// 単一年または単一年月からカレンダーオブジェクトを生成する
    fn single_calendar(&self, ym: &str) -> KoyomiResult<Calendar> {
        let splits = ym.split("-").collect::<Vec<_>>();
        match splits.len() {
            1 => {
                let from = Date::parse(&format!("{}-01-01", ym))?;
                let until = Date::parse(&format!("{}-12-31", ym))?;
                Ok(Calendar { from, until })
            }
            2 => {
                let fmt = format!("{}-{}-01", splits[0], splits[1]);
                let from = Date::parse(&fmt)?;
                let until = self.end_of_month(&fmt)?;
                Ok(Calendar { from, until })
            }
            _ => Err(KoyomiError::InvalidFormat(ym.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_of_month() {
        assert_eq!(num_days(2018, 1), 31);
        assert_eq!(num_days(2018, 2), 28);
        assert_eq!(num_days(2018, 3), 31);
        assert_eq!(num_days(2018, 4), 30);
        assert_eq!(num_days(2018, 5), 31);
        assert_eq!(num_days(2018, 6), 30);
        assert_eq!(num_days(2018, 7), 31);
        assert_eq!(num_days(2018, 8), 31);
        assert_eq!(num_days(2018, 9), 30);
        assert_eq!(num_days(2018, 10), 31);
        assert_eq!(num_days(2018, 11), 30);
        assert_eq!(num_days(2018, 12), 31);
    }

    #[test]
    fn days_of_february_with_leap() {
        assert_eq!(num_days(2016, 2), 29);
    }

    #[test]
    #[should_panic]
    fn days_of_invalid_month() {
        assert_eq!(num_days(2018, 13), 30);
    }

    #[test]
    fn leap_year() {
        // うるう年(4で割り切れる)
        assert!(is_leap(2016));
        // うるう年(100と400で割り切れる)
        assert!(is_leap(2000));
        // うるう年ではない(4で割り切れない)
        assert!(!is_leap(2017));
        // うるう年ではない(100で割り切れるが400で割り切れない)
        assert!(!is_leap(2100));
    }

    #[test]
    fn valid_calendar() {
        let from = Date::parse("2018-04-01").unwrap();
        let until = Date::parse("2018-04-30").unwrap();
        assert!(Calendar::new(from, until).is_ok());
    }

    #[test]
    fn invalid_calendar() {
        let from = Date::parse("2018-04-01").unwrap();
        let until = Date::parse("2017-04-01").unwrap();
        assert!(Calendar::new(from, until).is_err());
    }

    #[test]
    fn calendar_from() {
        let from = Date::parse("2018-04-01").unwrap();
        let until = Date::parse("2018-04-30").unwrap();
        let calendar = Calendar::new(from, until).unwrap();
        assert_eq!(calendar.from(), "2018-04-01");
    }

    #[test]
    fn make_calendar() {
        let from = Date::parse("2018-04-01").unwrap();
        let until = Date::parse("2018-04-30").unwrap();
        let cal = Calendar::new(from, until).unwrap().make();
        assert_eq!(cal.len(), 30);
        assert_eq!(cal[0].to_string(), "2018-04-01");
        assert_eq!(cal[29].to_string(), "2018-04-30");
    }

    #[test]
    fn calendar_until() {
        let from = Date::parse("2018-04-01").unwrap();
        let until = Date::parse("2018-04-30").unwrap();
        let calendar = Calendar::new(from, until).unwrap();
        assert_eq!(calendar.until(), "2018-04-30");
    }

    #[test]
    fn builder_single() {
        let c = Calendar::build().single("2018").finalize();
        assert!(c.is_ok());

        let c = Calendar::build().single("2018-04").finalize();
        assert!(c.is_ok());

        let c = Calendar::build().single("abc").finalize();
        assert!(c.is_err());
    }

    #[test]
    fn builder_between() {
        let c = Calendar::build().from("2017").until("2018").finalize();
        assert!(c.is_ok());

        let c = Calendar::build()
            .from("2017-04")
            .until("2018-04")
            .finalize();
        assert!(c.is_ok());

        let c = Calendar::build().from("2017").until("2018-04").finalize();
        assert!(c.is_ok());

        let c = Calendar::build().from("2017-04").until("2018").finalize();
        assert!(c.is_ok());

        let c = Calendar::build().from("abc").until("2018").finalize();
        assert!(c.is_err());

        let c = Calendar::build().from("2017").until("abc").finalize();
        assert!(c.is_err());
    }
}
