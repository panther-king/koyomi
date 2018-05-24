//! # 和暦定義
//!
//! 西暦に対応する元号(和暦)の定義。
//! 天皇即位日に元号が変わり、在位中は変わらない。
use date::Date;

/// 明治以降の和暦
const ERA: [(&str, i32, u32, u32, Option<(i32, u32, u32)>); 4] = [
    ("平成", 1989, 1, 8, None),
    ("昭和", 1926, 12, 25, Some((1989, 1, 7))),
    ("大正", 1912, 7, 30, Some((1926, 12, 24))),
    ("明治", 1868, 1, 25, Some((1912, 7, 29))),
];

/// 日付けから和暦を導出する
///
/// 即位日が改元日であり即位年は2つの和暦が混在するため、日付けから導出する。
/// 明治以前は対象外。
///
/// # Examples
///
/// ```rust
/// use koyomi::{Date, era};
///
/// let date = Date::from_ymd(2018, 1, 1).unwrap();
/// assert!(era(&date).is_some());
///
/// let date = Date::from_ymd(1800, 1, 1).unwrap();
/// assert!(era(&date).is_none());
/// ```
pub fn era(date: &Date) -> Option<Era> {
    // 大半は最新の元号に合致するユースケースを想定しているので、
    // 初回で抜けられるようループ処理で判定する。
    for &(name, y, m, d, until) in ERA.iter() {
        let from = Date::from_ymd(y, m, d).expect("Invalid era date!");
        let until =
            until.map(|(y, m, d)| Date::from_ymd(y, m, d).expect("Invalid until era date!"));
        let era = Era {
            ad: date.year(),
            name: name.into(),
            from,
            until,
        };
        if era.is_match(date) {
            return Some(era);
        }
    }

    None
}

/// 元号
///
/// https://ja.wikipedia.org/wiki/元号
#[derive(Debug, Eq, PartialEq)]
pub struct Era {
    ad: i32,
    name: String,
    from: Date,
    until: Option<Date>,
}

impl Era {
    /// 元号名を返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::{Date, era};
    ///
    /// let date = Date::from_ymd(2018, 1, 1).unwrap();
    /// let e = era(&date).unwrap();
    /// assert_eq!(e.name(), "平成");
    /// ```
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    /// 元号に基づいた「年」を返す
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::{Date, era};
    ///
    /// let date = Date::from_ymd(2018, 1, 1).unwrap();
    /// let e = era(&date).unwrap();
    /// assert_eq!(e.year(), 30);
    /// ```
    pub fn year(&self) -> i32 {
        self.ad - self.from.year() + 1
    }

    /// 元号の文字列表現を返す
    /// 初年度は「1年」ではなく「元年」
    ///
    /// # Examples
    ///
    /// ```rust
    /// use koyomi::{Date, era};
    ///
    /// let date = Date::from_ymd(2018, 1, 1).unwrap();
    /// let e = era(&date).unwrap();
    /// assert_eq!(e.format(), "平成30年");
    ///
    /// let date = Date::from_ymd(1989, 1, 10).unwrap();
    /// let e = era(&date).unwrap();
    /// assert_eq!(e.format(), "平成元年");
    /// ```
    pub fn format(&self) -> String {
        match self.year() {
            1 => format!("{}元年", self.name),
            y => format!("{}{}年", self.name, y),
        }
    }

    /// 日付けが自分の元号の範囲内かどうかを判定する
    fn is_match(&self, date: &Date) -> bool {
        match self.until {
            Some(ref until) => &self.from <= date && date <= until,
            None => &self.from <= date,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn era_name_meiji() {
        let date = Date::parse("1868-01-25").unwrap();
        assert_eq!(era(&date).unwrap().name(), "明治");

        let date = Date::parse("1912-07-29").unwrap();
        assert_eq!(era(&date).unwrap().name(), "明治");
    }

    #[test]
    fn era_year_meiji() {
        let date = Date::parse("1868-01-25").unwrap();
        assert_eq!(era(&date).unwrap().year(), 1);

        let date = Date::parse("1912-07-29").unwrap();
        assert_eq!(era(&date).unwrap().year(), 45);
    }

    #[test]
    fn era_format_meiji() {
        let date = Date::parse("1868-01-25").unwrap();
        assert_eq!(era(&date).unwrap().format(), "明治元年");

        let date = Date::parse("1912-07-29").unwrap();
        assert_eq!(era(&date).unwrap().format(), "明治45年");
    }

    #[test]
    fn era_name_taisho() {
        let date = Date::parse("1912-07-30").unwrap();
        assert_eq!(era(&date).unwrap().name(), "大正");

        let date = Date::parse("1926-12-24").unwrap();
        assert_eq!(era(&date).unwrap().name(), "大正");
    }

    #[test]
    fn era_year_taisho() {
        let date = Date::parse("1912-07-30").unwrap();
        assert_eq!(era(&date).unwrap().year(), 1);

        let date = Date::parse("1926-12-24").unwrap();
        assert_eq!(era(&date).unwrap().year(), 15);
    }

    #[test]
    fn era_format_taisho() {
        let date = Date::parse("1912-07-30").unwrap();
        assert_eq!(era(&date).unwrap().format(), "大正元年");

        let date = Date::parse("1926-12-24").unwrap();
        assert_eq!(era(&date).unwrap().format(), "大正15年");
    }

    #[test]
    fn era_name_showa() {
        let date = Date::parse("1926-12-25").unwrap();
        assert_eq!(era(&date).unwrap().name(), "昭和");

        let date = Date::parse("1989-01-07").unwrap();
        assert_eq!(era(&date).unwrap().name(), "昭和");
    }

    #[test]
    fn era_year_showa() {
        let date = Date::parse("1926-12-25").unwrap();
        assert_eq!(era(&date).unwrap().year(), 1);

        let date = Date::parse("1989-01-07").unwrap();
        assert_eq!(era(&date).unwrap().year(), 64);
    }

    #[test]
    fn era_format_showa() {
        let date = Date::parse("1926-12-25").unwrap();
        assert_eq!(era(&date).unwrap().format(), "昭和元年");

        let date = Date::parse("1989-01-07").unwrap();
        assert_eq!(era(&date).unwrap().format(), "昭和64年");
    }

    #[test]
    fn era_name_heisei() {
        let date = Date::parse("1989-01-08").unwrap();
        assert_eq!(era(&date).unwrap().name(), "平成");

        let date = Date::parse("2018-04-01").unwrap();
        assert_eq!(era(&date).unwrap().name(), "平成");
    }

    #[test]
    fn era_year_heisei() {
        let date = Date::parse("1989-01-08").unwrap();
        assert_eq!(era(&date).unwrap().year(), 1);

        let date = Date::parse("2018-04-01").unwrap();
        assert_eq!(era(&date).unwrap().year(), 30);
    }

    #[test]
    fn era_format_heisei() {
        let date = Date::parse("1989-01-08").unwrap();
        assert_eq!(era(&date).unwrap().format(), "平成元年");

        let date = Date::parse("2018-04-01").unwrap();
        assert_eq!(era(&date).unwrap().format(), "平成30年");
    }

    #[test]
    fn era_unknonw() {
        let date = Date::parse("1868-01-24").unwrap();
        assert!(era(&date).is_none());
    }
}
