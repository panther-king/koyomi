//! # 祝祭日定義
//!
//! 日本の法律で祝日・祭日となる日と、
//! 指定日が祝祭日にあたるかどうかを判定する
//! 関数を定義する。
use KoyomiResult;
use date::{Date, Weekday};

/// 指定日が祝祭日にあたるかどうかを判定する
///
/// # Examples
///
/// ```rust
/// use koyomi;
///
/// let date = koyomi::Date::from_ymd(2018, 1, 1).unwrap();
/// let holiday = koyomi::holiday(&date);
/// assert_eq!(holiday.unwrap(), "元日");
///
/// let date = koyomi::Date::from_ymd(2018, 1, 2).unwrap();
/// let holiday = koyomi::holiday(&date);
/// assert_eq!(holiday, None);
/// ```
pub fn holiday(date: &Date) -> Option<String> {
    // 規定の祝祭日
    defined_holiday(date)
        // 振替休日(前日が日曜で祝日)
        .or(substitute_holiday(date))
        // 成人の日(1月第2月曜)
        .or(variable_holiday(1, date, is_second_week))
        // 海の日(7月第3月曜)
        .or(variable_holiday(9, date, is_third_week))
        // 敬老の日(9月第3月曜)
        .or(variable_holiday(11, date, is_third_week))
        // 体育の日(10月第2月曜)
        .or(variable_holiday(12, date, is_second_week))
        // 春分の日
        .or(vernal_equinox_day(date))
        // 秋分の日
        .or(autumnal_equinox_day(date))
        // 国民の休日(前後が祝日の平日)
        .or(national_holiday(date))
}

/// 秋分日
/// @see https://ja.wikipedia.org/wiki/秋分の日
const AUTUMNAL_EQUINOX_DAYS: [[u32; 4]; 7] = [
    [23, 24, 24, 24], // 1900-1919
    [23, 23, 24, 24], // 1920-1947
    [23, 23, 23, 24], // 1948-1979
    [23, 23, 23, 23], // 1980-2011
    [22, 23, 23, 23], // 2012-2043
    [22, 22, 23, 23], // 2044-2075
    [22, 22, 22, 23], // 2076-2099
];

/// 国民の祝日
/// @see https://ja.wikipedia.org/wiki/国民の祝日
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
    ("天皇誕生日", 1989, 12, 23, None),
];

/// 春分日
/// @see https://ja.wikipedia.org/wiki/春分の日
const VERNAL_EQUINOX_DAYS: [[u32; 4]; 7] = [
    [21, 21, 21, 22], // 1900-1923
    [21, 21, 21, 21], // 1924-1959
    [20, 21, 21, 21], // 1960-1991
    [20, 20, 21, 21], // 1992-2023
    [20, 20, 20, 21], // 2024-2055
    [20, 20, 20, 20], // 2056-2091
    [19, 20, 20, 20], // 2092-2099
];

/// 国民の祝日に関する法律が施行された年
const HOLIDAY_FROM: i32 = 1948;

/// 国民の休日に関する法律が施行された年
const NATION_FROM: i32 = 1986;

/// 1週間は何日か？(指定日が何週目にあたるかの判定で利用する)
const ONE_WEEK: u32 = 7;

/// 振替休日に関する法律が施行された年
const SUBSTITUTE_FROM: i32 = 1973;

/// 指定日が秋分の日かどうかを判定する
fn autumnal_equinox_day(date: &Date) -> Option<String> {
    if date.year() < HOLIDAY_FROM {
        return None;
    }

    if date.month() != 9 {
        return None;
    }

    let index = match date.year() {
        1900...1919 => Some(0),
        1920...1947 => Some(1),
        1948...1979 => Some(2),
        1980...2011 => Some(3),
        2012...2043 => Some(4),
        2044...2075 => Some(5),
        2076...2099 => Some(6),
        _ => None,
    };

    match index {
        None => None,
        Some(i) => {
            let m = date.year() % 4;
            let d = AUTUMNAL_EQUINOX_DAYS[i as usize][m as usize];
            if date.day() == d {
                Some("秋分の日".into())
            } else {
                None
            }
        }
    }
}

/// 指定日が定義された祝祭日かどうかを判定する
fn defined_holiday(date: &Date) -> Option<String> {
    if date.year() < HOLIDAY_FROM {
        return None;
    }

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

/// 指定日が第2週にあたるかどうかを判定する
/// 第2週の月曜日となった、成人の日・体育の日を判定するために利用する
fn is_second_week(day: u32) -> bool {
    (day / ONE_WEEK == 2 && day % ONE_WEEK == 0) || (day / ONE_WEEK == 1 && day % ONE_WEEK > 0)
}

/// 指定日が第3週にあたるかどうかを判定する
/// 第3週の月曜日となった、海の日・敬老の日を判定するために利用する
fn is_third_week(day: u32) -> bool {
    (day / ONE_WEEK == 3 && day % ONE_WEEK == 0) || (day / ONE_WEEK == 2 && day % ONE_WEEK >= 1)
}

/// 指定日が国民の休日にあたるかどうかを判定する
fn national_holiday(date: &Date) -> Option<String> {
    if date.year() < NATION_FROM {
        return None;
    }

    if date.weekday() == &Weekday::Sunday {
        return None;
    }

    if defined_holiday(&date).is_some() {
        return None;
    }

    // シルバーウィークで、敬老の日(可変)を考慮する必要がある
    let yesterday = date.yesterday().ok()?;
    let between = defined_holiday(&yesterday).or(variable_holiday(11, &yesterday, is_third_week));
    if between.is_none() {
        return None;
    }

    // シルバーウィークで、秋分の日を考慮する必要がある
    let tomorrow = date.tomorrow().ok()?;
    let between = defined_holiday(&tomorrow).or(autumnal_equinox_day(&tomorrow));
    if between.is_none() {
        return None;
    }

    Some("国民の休日".into())
}

/// 指定日が振替休日かどうかを判定する
///
/// 日曜が祝日の場合は、その次の平日が振替休日となるため、
/// 遡って判定する必要がある(月曜日とは限らない)
fn substitute(yesterday: KoyomiResult<Date>) -> Option<String> {
    match yesterday {
        Err(_) => None,
        Ok(y) => {
            let holiday = defined_holiday(&y)
                .or(vernal_equinox_day(&y))
                .or(autumnal_equinox_day(&y));
            match holiday {
                None => None,
                Some(_) if y.weekday() == &Weekday::Sunday => Some("振替休日".into()),
                Some(_) => substitute(y.yesterday()),
            }
        }
    }
}

/// 指定日が振替休日かどうかを判定する
fn substitute_holiday(date: &Date) -> Option<String> {
    if date.year() < SUBSTITUTE_FROM {
        None
    } else {
        substitute(date.yesterday())
    }
}

/// 指定日が年ごとに変動する祝日かどうかを判定する
fn variable_holiday(index: usize, date: &Date, week: impl Fn(u32) -> bool) -> Option<String> {
    if date.month() != HOLIDAYS[index].2 {
        return None;
    }

    if date.weekday() != &Weekday::Monday {
        return None;
    }

    if date.year() <= HOLIDAYS[index].4.unwrap() {
        return None;
    }

    if !week(date.day()) {
        return None;
    }

    Some(HOLIDAYS[index].0.into())
}

/// 指定日が春分の日かどうかを判定する
fn vernal_equinox_day(date: &Date) -> Option<String> {
    if date.year() <= HOLIDAY_FROM {
        return None;
    }

    if date.month() != 3 {
        return None;
    }

    let index = match date.year() {
        1900...1923 => Some(0),
        1924...1959 => Some(1),
        1960...1991 => Some(2),
        1992...2023 => Some(3),
        2024...2055 => Some(4),
        2056...2091 => Some(5),
        2092...2099 => Some(6),
        _ => None,
    };

    match index {
        None => None,
        Some(i) => {
            let m = date.year() % 4;
            let d = VERNAL_EQUINOX_DAYS[i as usize][m as usize];
            if date.day() == d {
                Some("春分の日".into())
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_years_day() {
        let name = "元日";

        let date = Date::from_ymd(1948, 1, 1).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1947, 1, 1).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn fixed_coming_of_age() {
        let name = "成人の日";

        let date = Date::from_ymd(1948, 1, 15).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1999, 1, 15).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1947, 1, 15).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(2000, 1, 15).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn variable_coming_of_age() {
        let name = "成人の日";

        let date = Date::from_ymd(2000, 1, 10).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1999, 1, 11).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(2018, 1, 9).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(2018, 3, 12).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn national_foundation_day() {
        let name = "建国記念日";

        let date = Date::from_ymd(1967, 2, 11).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1966, 2, 11).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn birthday_of_showa_emperor() {
        let name = "天皇誕生日";

        let date = Date::from_ymd(1948, 4, 29).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1988, 4, 29).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1947, 4, 29).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn green_day() {
        let name = "みどりの日";

        let date = Date::from_ymd(1989, 4, 29).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2006, 4, 29).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2007, 5, 4).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2006, 5, 4).unwrap();
        assert_ne!(holiday(&date).unwrap(), name);
    }

    #[test]
    fn showa_day() {
        let name = "昭和の日";

        let date = Date::from_ymd(2007, 4, 29).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);
    }

    #[test]
    fn constitution_day() {
        let name = "憲法記念日";

        let date = Date::from_ymd(1948, 5, 3).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1947, 5, 3).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn childrens_day() {
        let name = "こどもの日";

        let date = Date::from_ymd(1948, 5, 5).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1947, 5, 5).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn fixed_marine_day() {
        let name = "海の日";

        let date = Date::from_ymd(1996, 7, 20).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2002, 7, 20).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1995, 7, 20).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(2003, 7, 20).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn variable_marine_day() {
        let name = "海の日";

        let date = Date::from_ymd(2003, 7, 21).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2002, 7, 15).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(2018, 7, 17).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(2018, 8, 20).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn mountain_day() {
        let name = "山の日";

        let date = Date::from_ymd(2016, 8, 11).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2015, 8, 11).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn fixed_respect_for_the_aged_day() {
        let name = "敬老の日";

        let date = Date::from_ymd(1966, 9, 15).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2002, 9, 15).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1965, 9, 15).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(2004, 9, 15).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn variable_respect_for_the_aged_day() {
        let name = "敬老の日";

        let date = Date::from_ymd(2003, 9, 15).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2002, 9, 16).unwrap();
        assert_ne!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2018, 9, 18).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(2018, 6, 18).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn fixed_sports_day() {
        let name = "体育の日";

        let date = Date::from_ymd(1966, 10, 10).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1999, 10, 10).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1965, 10, 10).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(2000, 10, 10).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn variable_sports_day() {
        let name = "体育の日";

        let date = Date::from_ymd(2000, 10, 9).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1999, 10, 11).unwrap();
        assert_ne!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2018, 10, 9).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(2018, 11, 12).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn culture_day() {
        let name = "文化の日";

        let date = Date::from_ymd(1948, 11, 3).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1947, 11, 3).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn labor_thanksgiving_day() {
        let name = "勤労感謝の日";

        let date = Date::from_ymd(1948, 11, 23).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1947, 11, 23).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn birthday_of_heisei_emperor() {
        let name = "天皇誕生日";

        let date = Date::from_ymd(1989, 12, 23).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1988, 12, 23).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn substitute_holiday() {
        let name = "振替休日";

        let date = Date::from_ymd(2018, 4, 30).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2018, 4, 23).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn vernal_equinox_day() {
        let name = "春分の日";

        let date = Date::from_ymd(2018, 3, 21).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2018, 2, 21).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(1899, 3, 20).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(2300, 3, 20).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn autumnal_equinox_day() {
        let name = "秋分の日";

        let date = Date::from_ymd(2018, 9, 23).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2018, 8, 23).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(1899, 9, 23).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(2300, 9, 23).unwrap();
        assert!(holiday(&date).is_none());
    }

    #[test]
    fn national_holiday() {
        let name = "国民の休日";

        let date = Date::from_ymd(1988, 5, 4).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(1986, 5, 4).unwrap();
        assert!(holiday(&date).is_none());

        let date = Date::from_ymd(1987, 5, 4).unwrap();
        assert_ne!(holiday(&date).unwrap(), name);
    }
}
