koyomi
======

[![Build Status](https://travis-ci.org/panther-king/koyomi.svg?branch=master)](https://travis-ci.org/panther-king/koyomi) [![Crates.io](https://img.shields.io/crates/v/koyomi.svg)](https://crates.io/crates/koyomi) [![Crates.io](https://img.shields.io/crates/d/koyomi.svg)](https://crates.io/crates/koyomi) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/panther-king/koyomi/blob/master/LICENSE)

__This library is no longer maintained. Please consider using [koyomi-rs](https://github.com/panther-king/koyomi-rs) instead.__

This is a calendar for Japanese.

About
=====

- Generate a calendar
    - Specified a year or year-and-month.
        - e.g. `2018`
        - e.g. `2018-01`
    - Specified between `from` and `until`.
        - e.g. `2018-01` and `2018-12`
- Generate a date
    - `Date` has year, month, day, weekday and below.
        - Japanese Calendar
        - Japanese weekday
        - Japanese holiday

Usage
=====

Add `koyomi` as a dependency in your `Cargo.toml`

```toml
[dependencies]
koyomi = "0.3"
```

Quick Example
=============

`Date` can be initialized from `&str` or `tuple(i32, u32, u32)`.

```rust
extern crate koyomi;

use koyomi::Date;

// Only "Y-m-d" format
let date = Date::parse("2018-01-01").unwrap();
println!("{}", date); // 2018-01-01

// Same as above
let date = Date::from_ymd(2018, 1, 1).unwrap();
println!("{}", date); // 2018-01-01
```

`Calendar` can be initialized from `Date` or `CalendarBuilder`.

``` rust
extern crate koyomi;

use koyomi::{Calendar, Date};

// From `Date`
let from = Date::from_ymd(2018, 1, 1).unwrap();
let until = Date::from_ymd(2018, 12, 31).unwrap();
let calendar = Calendar::new(from, until).unwrap().make();

println!("{}", calendar.len()); // 365
println!("{}", calendar[0]); // 2018-01-01

// From `CalendarBuilder`
let calendar = Calendar::build()
    .from("2018-01")
    .until("2018-12")
    .finalize()
    .unwrap()
    .make();
println!("{}", calendar.len()); // 365
println!("{}", calrndar[0]); // 2018-01-01
```
