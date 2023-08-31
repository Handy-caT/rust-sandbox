extern crate core;

use time::{Date, Month};
use time::macros::format_description;


fn main() {
    println!("Implement me!");
}

const NOW: &str = "2019-06-26";

fn more_days(month_l: u8, month_r: u8, day_l: u8, day_r: u8) -> bool {
    if month_l > month_r {
        true
    } else if month_l == month_r {
        day_l > day_r
    } else {
        false
    }
}

struct User
{
    birthdate: Date,
}

impl User {
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        let month_num = u8::try_from(month).unwrap();
        let day_num = u8::try_from(day).unwrap();

        let month_enum = Month::try_from(month_num).unwrap();

        let date = Date::from_calendar_date(year, month_enum, day_num).unwrap();

        User {
            birthdate: date,
        }
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        let format = format_description!("[year]-[month]-[day]");
        let now = Date::parse(NOW, &format).unwrap();


        let age_years = now.year() - self.birthdate.year();

        if age_years <= 0 {
            0
        } else if more_days(now.month() as u8, self.birthdate.month() as u8, now.day(), self.birthdate.day()) {
            age_years as u16
        } else {
            (age_years - 1) as u16
        }

    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            ((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in vec![
            ((2019, 6, 27), 0),
            ((2032, 6, 25), 0),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn adult_check() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), true),
            ((1990, 7, 4), true),
            ((0, 1, 1), true),
            ((1970, 1, 1), true),
            ((2019, 6, 25), false),
            ((2019, 6, 26), false),
            ((2019, 6, 27), false),
            ((2032, 6, 25), false),
            ((3000, 6, 27), false),
            ((9999, 6, 27), false),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.is_adult(), expected);
        }
    }
}
