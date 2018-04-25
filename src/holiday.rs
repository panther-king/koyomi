//! 祝祭日定義
use date::{Date, Weekday};

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

const HOLIDAY_FROM: i32 = 1948;

const NATION_FROM: i32 = 1988;

const ONE_WEEK: u32 = 7;

const SUBSTITUTE_FROM: i32 = 1973;

pub fn holiday(date: &Date) -> Option<String> {
    defined_holiday(date)
        // 振替休日(前日が日曜で祝日)
        .or(substitude_holiday(date))
        // 成人の日(1月第2月曜)
        .or(variable_holiday(1, date, &is_second_week))
        // 海の日(7月第3月曜)
        .or(variable_holiday(9, date, &is_third_week))
        // 敬老の日(9月第3月曜)
        .or(variable_holiday(11, date, &is_third_week))
        // 体育の日(10月第2月曜)
        .or(variable_holiday(12, date, &is_second_week))
}

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

fn is_second_week(day: u32) -> bool {
    (day / ONE_WEEK == 2 && day % ONE_WEEK == 0) || (day / ONE_WEEK == 1 && day % ONE_WEEK > 0)
}

fn is_third_week(day: u32) -> bool {
    (day / ONE_WEEK == 3) || (day / ONE_WEEK == 2 && day % ONE_WEEK >= 1)
}

fn substitude_holiday(date: &Date) -> Option<String> {
    if date.year() < SUBSTITUTE_FROM {
        return None;
    }

    if date.weekday() != &Weekday::Monday {
        return None;
    }

    match date.yesterday() {
        Ok(yesterday) => defined_holiday(&yesterday).map(|_| "振替休日".into()),
        Err(_) => None,
    }
}

fn variable_holiday(index: usize, date: &Date, week: &Fn(u32) -> bool) -> Option<String> {
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
        assert!(holiday(&date).is_none());
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
    fn substitude_holiday() {
        let name = "振替休日";

        let date = Date::from_ymd(2018, 4, 30).unwrap();
        assert_eq!(holiday(&date).unwrap(), name);

        let date = Date::from_ymd(2018, 4, 23).unwrap();
        assert!(holiday(&date).is_none());
    }
}
