//! # 祝祭日定義
use date::Date;

/// # 祝祭日定義
///
/// 法律で定められた祝祭日の定義
/// `(名称, 月, 日, 開始年, 終了年)` のタプルで定義する。
///
/// - https://ja.wikipedia.org/wiki/国民の祝日
const HOLIDAYS: [(&str, u32, u32, i32, Option<i32>); 16] = [
    ("元日", 1, 1, 1948, None),
    ("成人の日", 1, 15, 1948, Some(1999)),
    ("建国記念日", 2, 11, 1967, None),
    ("天皇誕生日", 4, 29, 1948, Some(1988)),
    ("みどりの日", 4, 29, 1989, Some(2006)),
    ("昭和の日", 4, 29, 2007, None),
    ("憲法記念日", 5, 3, 1948, None),
    ("みどりの日", 5, 4, 2007, None),
    ("こどもの日", 5, 5, 1948, None),
    ("海の日", 7, 20, 1996, Some(2002)),
    ("山の日", 8, 11, 2016, None),
    ("敬老の日", 9, 15, 1966, Some(2002)),
    ("体育の日", 10, 10, 1966, Some(1999)),
    ("文化の日", 11, 3, 1948, None),
    ("勤労感謝の日", 11, 23, 1948, None),
    ("天皇誕生日", 12, 23, 1989, Some(2018)),
];

/// 指定日が祝祭日であるか判断する
pub fn holiday(date: &Date) -> Option<String> {
    HOLIDAYS
        .iter()
        .filter(|h| match h.4 {
            Some(until) => h.3 <= date.year() && date.year() <= until,
            None => h.3 <= date.year(),
        })
        .filter(|h| date.month() == h.1 && date.day() == h.2)
        .nth(0)
        .map(|h| h.0.to_owned())
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
}
