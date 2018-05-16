extern crate chrono;
extern crate koyomi;

use chrono::Datelike;

macro_rules! assert_holiday {
    ($y:expr, $m:expr, $d:expr, $expect:expr) => {
        let d = koyomi::Date::from_ymd($y, $m, $d).unwrap();
        assert_eq!(d.holiday().unwrap(), $expect);
    };

    ($name:expr, $days:expr) => {
        $days.iter().for_each(|&(y, m, d)| {
            assert_holiday!(y, m, d, $name);
        });
    };
}

fn years(from: i32) -> impl Iterator<Item = i32> {
    let current = chrono::Utc::now().year();

    (from..=current)
}

#[test]
fn new_years_day() {
    let name = "元日";

    years(1948).for_each(|y| {
        assert_holiday!(y, 1, 1, name);
    });
}

#[test]
fn coming_of_age_day() {
    let name = "成人の日";

    (1948..=1999).for_each(|y| {
        assert_holiday!(y, 1, 15, name);
    });

    assert_holiday!(
        name,
        vec![
            (2000, 1, 10),
            (2001, 1, 8),
            (2002, 1, 14),
            (2003, 1, 13),
            (2004, 1, 12),
            (2005, 1, 10),
            (2006, 1, 9),
            (2007, 1, 8),
            (2008, 1, 14),
            (2009, 1, 12),
            (2010, 1, 11),
            (2011, 1, 10),
            (2012, 1, 9),
            (2013, 1, 14),
            (2014, 1, 13),
            (2015, 1, 12),
            (2016, 1, 11),
            (2017, 1, 9),
            (2018, 1, 8),
        ]
    );
}

#[test]
fn national_foundation_day() {
    let name = "建国記念日";

    years(1967).for_each(|y| {
        assert_holiday!(y, 2, 11, name);
    });
}

// 春分の日

#[test]
fn birthday_of_showa_emperor() {
    let name = "天皇誕生日";

    (1948..=1988).for_each(|y| {
        assert_holiday!(y, 4, 29, name);
    });
}

#[test]
fn green_day() {
    let name = "みどりの日";

    (1989..=2006).for_each(|y| {
        assert_holiday!(y, 4, 29, name);
    });

    years(2007).for_each(|y| {
        assert_holiday!(y, 5, 4, name);
    });
}

#[test]
fn showa_day() {
    let name = "昭和の日";

    years(2007).for_each(|y| {
        assert_holiday!(y, 4, 29, name);
    });
}

#[test]
fn constitution_day() {
    let name = "憲法記念日";

    years(1948).for_each(|y| {
        assert_holiday!(y, 5, 3, name);
    });
}

#[test]
fn childrens_day() {
    let name = "こどもの日";

    years(1948).for_each(|y| {
        assert_holiday!(y, 5, 5, name);
    });
}

#[test]
fn marine_day() {
    let name = "海の日";

    (1996..=2002).for_each(|y| {
        assert_holiday!(y, 7, 20, name);
    });

    assert_holiday!(
        name,
        vec![
            (2003, 7, 21),
            (2004, 7, 19),
            (2005, 7, 18),
            (2006, 7, 17),
            (2007, 7, 16),
            (2008, 7, 21),
            (2009, 7, 20),
            (2010, 7, 19),
            (2011, 7, 18),
            (2012, 7, 16),
            (2013, 7, 15),
            (2014, 7, 21),
            (2015, 7, 20),
            (2016, 7, 18),
            (2017, 7, 17),
            (2018, 7, 16),
        ]
    );
}

#[test]
fn mountain_day() {
    let name = "山の日";

    years(2016).for_each(|y| {
        assert_holiday!(y, 8, 11, name);
    });
}

#[test]
fn respect_for_the_aged_day() {
    let name = "敬老の日";

    (1966..=2002).for_each(|y| {
        assert_holiday!(y, 9, 15, name);
    });

    assert_holiday!(
        name,
        vec![
            (2003, 9, 15),
            (2004, 9, 20),
            (2005, 9, 19),
            (2006, 9, 18),
            (2007, 9, 17),
            (2008, 9, 15),
            (2009, 9, 21),
            (2010, 9, 20),
            (2011, 9, 19),
            (2012, 9, 17),
            (2013, 9, 16),
            (2014, 9, 15),
            (2015, 9, 21),
            (2016, 9, 19),
            (2017, 9, 18),
            (2018, 9, 17),
        ]
    );
}

// 秋分の日

#[test]
fn sports_day() {
    let name = "体育の日";

    (1966..=1999).for_each(|y| {
        assert_holiday!(y, 10, 10, name);
    });

    assert_holiday!(
        name,
        vec![
            (2000, 10, 9),
            (2001, 10, 8),
            (2002, 10, 14),
            (2003, 10, 13),
            (2004, 10, 11),
            (2005, 10, 10),
            (2006, 10, 9),
            (2007, 10, 8),
            (2008, 10, 13),
            (2009, 10, 12),
            (2010, 10, 11),
            (2011, 10, 10),
            (2012, 10, 8),
            (2013, 10, 14),
            (2014, 10, 13),
            (2015, 10, 12),
            (2016, 10, 10),
            (2017, 10, 9),
            (2018, 10, 8),
        ]
    );
}

#[test]
fn culture_day() {
    let name = "文化の日";

    years(1948).for_each(|y| {
        assert_holiday!(y, 11, 3, name);
    });
}

#[test]
fn labor_thanksgiving_day() {
    let name = "勤労感謝の日";

    years(1948).for_each(|y| {
        assert_holiday!(y, 11, 23, name);
    });
}

#[test]
fn birthday_of_heisei_emperor() {
    let name = "天皇誕生日";

    years(1989).for_each(|y| {
        assert_holiday!(y, 12, 23, name);
    });
}
