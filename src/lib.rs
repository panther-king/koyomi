extern crate chrono;

mod calendar;
mod date;
mod era;
mod holiday;

#[derive(Debug)]
pub enum KoyomiError {
    InvalidDate(String),
    NoTomorrow(i32, u32, u32),
    NoYesterday(i32, u32, u32),
}
