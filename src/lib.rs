extern crate chrono;

#[cfg(test)]
#[macro_use()]
extern crate proptest;


use proptest::*;
use chrono::NaiveDate;

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError,
    InvalidDateError,
    InvalidDayError,
    InvalidMonthError,
}

impl From<std::num::ParseIntError> for Error {
    fn from(T: std::num::ParseIntError) -> Error {
        Error::ParseError
    }
}
/// Parses a human-readable date format
/// e.g.
/// 4th of July 1987
/// 3rd of August 2019
/// 2nd of March 2006
/// 
/// 
pub fn parse_date(human_date: impl AsRef<str>) -> Result<chrono::NaiveDate, Error> {
    let parts: Vec<&str> = human_date.as_ref().split_whitespace().collect();

    if parts.len() != 4 {
        return Err(Error::ParseError);
    }
    let ordinal_day = parts[0];

    let day: u32 = parse_day(ordinal_day)?;
    let month: u32 = parse_month(parts[2])?;
    let year: i32 = parts[3].parse()?;

    NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| Error::InvalidDateError)
}

fn parse_day(day_with_ordinal: &str) -> Result<u32, Error> {
     let day: u32 = day_with_ordinal.chars().take_while(|c| c.is_digit(10)).collect::<String>().parse()?;
    let ordinal = day_with_ordinal.chars().skip_while(|c| c.is_digit(10)).collect::<String>();
    match (day, ordinal.as_ref()) {
        (1, "st") |
        (2, "nd") |
        (3, "rd") |
        (4..=20, "th") |
        (21, "st") |
        (22, "nd") |
        (23, "rd") |
        (24..=30, "th") |
        (31, "st") => Ok(day),  // returns Result(Ok(dat:u3s))
        _ => Err(Error::InvalidDayError)  // Error tuple structure as result
    }
}


fn parse_month(month : &str) -> Result<u32, Error> {
    let months = vec!["January", "February","March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    match months.iter().position(|&elem| elem == month) {
        Some(index) => Ok(index as u32 + 1),
        _ => Err(Error::InvalidMonthError)
    }
}



mod tests {
use crate ::parse_date;
    #[test]
    fn it_works() {
        assert_eq!(
            parse_date("4th of September 2015"), 
            Ok(chrono::NaiveDate::from_ymd(2015, 9, 4))
        );

        assert_eq!(
            parse_date("5th of September 2015"), 
            Ok(chrono::NaiveDate::from_ymd(2015, 9, 5))
        );
        assert_eq!(
            parse_date("25th of December 2015"), 
            Ok(chrono::NaiveDate::from_ymd(2015, 12, 25))
        );
    }
}

    proptest! {
    #[test]
    fn doesent_crash(ref s in "\\PC*") {
        parse_date(s);
    }


    #[test]
    fn parse_date_to_original(y in 0i32..10000, m in 1u32..=12, d in 1u32..=31) {
        // Convert y, d, m to Dth of MONTH YEAR
        let months = vec!["January", "February","March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

        let ordinal_suffix = match d {
            1| 21| 31 => "st",
            2|22 => "nd",
            3|23 => "rd",
            _ => "th",
        };
        let date_string = format!("{}{} of {} {}", d, ordinal_suffix, months[(m -1) as usize], y);
        println!("y: {}  d: {} m: {}, string: {}", y, d, m, date_string);
        assert_eq!(parse_date(date_string), chrono::NaiveDate::from_ymd_opt(y, m, d).ok_or_else(|| Error::InvalidDateError))
    }

    #[test]
    fn doesent_crash2(ref s in "([0-9a-z]{1,5}){3}[0-9a-z]{1,5}") {
        parse_date(s);
    }

}