//! # カレンダー
pub fn num_days(year: i32, month: u32) -> u8 {
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
}
