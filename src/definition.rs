//! # 祝祭日定義
use date::Date;

/// # 祝祭日定義
///
/// 法律で定められた祝祭日の定義
/// `(名称, 開始年, 月, 日, 終了年)` のフォーマットで定義する。
///
/// - https://ja.wikipedia.org/wiki/国民の祝日
const HOLIDAYS: [(&str, i32, u32, u32, Option<i32>); 16] = [
    ("元日", 1948, 1, 1, None),
    ("成人の日", 1948, 1, 15, Some(1999)),
    ("建国記念日", 1967, 2, 11, None),
    ("天皇誕生日", 1948, 4, 29, Some(1988)),
    ("みどりの日", 1989, 4, 29, Some(2006)),
    ("昭和の日", 2007, 4, 29, None),
    ("憲法記念日", 1948, 5, 3, None),
    ("みどりの日", 2007, 5, 4, None),
    ("こどもの日", 1948, 5, 5, None),
    ("海の日", 1996, 7, 20, Some(2002)),
    ("山の日", 2016, 8, 11, None),
    ("敬老の日", 1966, 9, 15, Some(2002)),
    ("体育の日", 1966, 10, 10, Some(1999)),
    ("文化の日", 1948, 11, 3, None),
    ("勤労感謝の日", 1948, 11, 23, None),
    ("天皇誕生日", 1989, 12, 23, Some(2018)),
];

/// 指定日が祝祭日・振替休日であるか判定する
pub fn holiday(date: &Date) -> Option<String> {
    substitude_holiday(date).or(defined_holiday(date))
}

/// 指定日が祝祭日であるか判定する
fn defined_holiday(date: &Date) -> Option<String> {
    HOLIDAYS
        .iter()
        .filter(|h| match h.4 {
            Some(until) => h.1 <= date.year() && date.year() <= until,
            None => h.1 <= date.year(),
        })
        .filter(|h| date.month() == h.2 && date.day() == h.3)
        .nth(0)
        .map(|h| h.0.into())
}

/// 指定日が振替休日であるか判定する
fn substitude_holiday(date: &Date) -> Option<String> {
    if date.year() < 1973 || date.weekday() != "月" {
        return None;
    }

    match date.prev() {
        Some(yesterday) => match defined_holiday(&yesterday) {
            Some(_) => Some("振替休日".into()),
            None => None,
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_year_day() {
        let name = "元日";

        let d = Date::parse("1948-01-01").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1947-01-01").unwrap();
        assert_eq!(holiday(&d), None);
    }

    #[test]
    fn coming_of_age() {
        let name = "成人の日";

        let d = Date::parse("1948-01-15").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1999-01-15").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1947-01-15").unwrap();
        assert_eq!(holiday(&d), None);

        let d = Date::parse("2000-01-15").unwrap();
        assert_eq!(holiday(&d), None);
    }

    #[test]
    fn national_foundation() {
        let name = "建国記念日";

        let d = Date::parse("1967-02-11").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1966-02-11").unwrap();
        assert_eq!(holiday(&d), None);
    }

    #[test]
    fn emperor_birthday() {
        let name = "天皇誕生日";

        let d = Date::parse("1948-04-29").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1988-04-29").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1947-04-29").unwrap();
        assert_eq!(holiday(&d), None);

        let d = Date::parse("1989-04-29").unwrap();
        assert!(holiday(&d).unwrap() != name);

        let d = Date::parse("1989-12-23").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("2019-12-23").unwrap();
        assert_eq!(holiday(&d), None);

        let d = Date::parse("1988-12-23").unwrap();
        assert_eq!(holiday(&d), None);

        let d = Date::parse("2020-12-23").unwrap();
        assert_eq!(holiday(&d), None);
    }

    #[test]
    fn greenery_day() {
        let name = "みどりの日";

        let d = Date::parse("1989-04-29").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("2006-04-29").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1988-04-29").unwrap();
        assert!(holiday(&d).unwrap() != name);

        let d = Date::parse("2007-04-29").unwrap();
        assert!(holiday(&d).unwrap() != name);

        let d = Date::parse("2007-05-04").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("2006-05-04").unwrap();
        assert_eq!(holiday(&d), None);
    }

    #[test]
    fn showa_day() {
        let name = "昭和の日";

        let d = Date::parse("2007-04-29").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("2006-04-29").unwrap();
        assert!(holiday(&d).unwrap() != name);
    }

    #[test]
    fn constitution_day() {
        let name = "憲法記念日";

        let d = Date::parse("1948-05-03").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1947-05-03").unwrap();
        assert_eq!(holiday(&d), None);
    }

    #[test]
    fn children_day() {
        let name = "こどもの日";

        let d = Date::parse("1948-05-05").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1947-05-05").unwrap();
        assert_eq!(holiday(&d), None);
    }

    #[test]
    fn marine_day() {
        let name = "海の日";

        let d = Date::parse("1996-07-20").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("2002-07-20").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1995-07-20").unwrap();
        assert_eq!(holiday(&d), None);

        let d = Date::parse("2003-07-20").unwrap();
        assert_eq!(holiday(&d), None);
    }

    #[test]
    fn mountain_day() {
        let name = "山の日";

        let d = Date::parse("2016-08-11").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("2015-08-11").unwrap();
        assert_eq!(holiday(&d), None);
    }

    #[test]
    fn respect_for_the_aged_day() {
        let name = "敬老の日";

        let d = Date::parse("1966-09-15").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("2002-09-15").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1965-09-15").unwrap();
        assert_eq!(holiday(&d), None);

        let d = Date::parse("2003-09-15").unwrap();
        assert_eq!(holiday(&d), None);
    }

    #[test]
    fn sports_day() {
        let name = "体育の日";

        let d = Date::parse("1966-10-10").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1999-10-10").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1965-10-10").unwrap();
        assert_eq!(holiday(&d), None);

        let d = Date::parse("2000-10-10").unwrap();
        assert_eq!(holiday(&d), None);
    }

    #[test]
    fn culture_day() {
        let name = "文化の日";

        let d = Date::parse("1948-11-03").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1947-11-03").unwrap();
        assert_eq!(holiday(&d), None);
    }

    #[test]
    fn labor_thanksgiving_day() {
        let name = "勤労感謝の日";

        let d = Date::parse("1948-11-23").unwrap();
        assert_eq!(holiday(&d), Some(name.into()));

        let d = Date::parse("1947-11-23").unwrap();
        assert_eq!(holiday(&d), None);
    }

    #[test]
    fn substitude_holiday() {
        let d = Date::parse("2018-04-30").unwrap();
        assert_eq!(holiday(&d), Some("振替休日".into()));
    }
}
