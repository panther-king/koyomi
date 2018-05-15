extern crate koyomi;

macro_rules! assert_koyomi {
    ($e1:expr, $e2:expr, $e3:expr, $e4:expr) => {
        assert_eq!($e1.to_string(), $e2);
        assert_eq!($e1.weekday().japanese(), $e3);
        assert_eq!($e1.holiday(), $e4);
    };
}

pub fn year_of_calendar(year: usize) -> Vec<koyomi::Date> {
    let year = format!("{}", year);
    let calendar = koyomi::Calendar::build().single(&year).finalize().unwrap();
    calendar.make()
}
