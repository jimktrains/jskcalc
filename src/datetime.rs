use std::cmp::Ordering;
use std::ops::Add;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Month {
    pub fn non_leap_year_start_doy_offset(self: &Self) -> u16 {
        match self {
            Month::Jan => 0,
            Month::Feb => 31,
            Month::Mar => 31 + 28,
            Month::Apr => 31 + 28 + 31,
            Month::May => 31 + 28 + 31 + 30,
            Month::Jun => 31 + 28 + 31 + 30 + 31,
            Month::Jul => 31 + 28 + 31 + 30 + 31 + 30,
            Month::Aug => 31 + 28 + 31 + 30 + 31 + 30 + 31,
            Month::Sep => 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31,
            Month::Oct => 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30,
            Month::Nov => 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31,
            Month::Dec => 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30,
        }
    }

    pub fn non_leap_year_days_in_month(self: &Self) -> u8 {
        match self {
            Month::Jan => 31,
            Month::Feb => 28,
            Month::Mar => 31,
            Month::Apr => 30,
            Month::May => 31,
            Month::Jun => 30,
            Month::Jul => 31,
            Month::Aug => 31,
            Month::Sep => 30,
            Month::Oct => 31,
            Month::Nov => 30,
            Month::Dec => 31,
        }
    }
}

impl From<u16> for Month {
    fn from(x: u16) -> Month {
        match x % 12 {
            0 => Month::Jan,
            1 => Month::Feb,
            2 => Month::Mar,
            3 => Month::Apr,
            4 => Month::May,
            5 => Month::Jun,
            6 => Month::Jul,
            7 => Month::Aug,
            8 => Month::Sep,
            9 => Month::Oct,
            10 => Month::Nov,
            11 => Month::Dec,
            _ => panic!("{} isn't a valid month number", x),
        }
    }
}
impl From<i16> for Month {
    fn from(x: i16) -> Month {
        let mut x = x;
        if x < 0 {
            x += 7
        }
        Month::from(x as u16)
    }
}

impl Into<u16> for &Month {
    fn into(self: Self) -> u16 {
        match self {
            Month::Jan => 0,
            Month::Feb => 1,
            Month::Mar => 2,
            Month::Apr => 3,
            Month::May => 4,
            Month::Jun => 5,
            Month::Jul => 6,
            Month::Aug => 7,
            Month::Sep => 8,
            Month::Oct => 9,
            Month::Nov => 10,
            Month::Dec => 11,
        }
    }
}

impl Into<u16> for Month {
    fn into(self: Self) -> u16 {
        match self {
            Month::Jan => 0,
            Month::Feb => 1,
            Month::Mar => 2,
            Month::Apr => 3,
            Month::May => 4,
            Month::Jun => 5,
            Month::Jul => 6,
            Month::Aug => 7,
            Month::Sep => 8,
            Month::Oct => 9,
            Month::Nov => 10,
            Month::Dec => 11,
        }
    }
}

impl Add<u16> for &Month {
    type Output = Month;
    fn add(self: Self, rhs: u16) -> Month {
        let x: u16 = self.into();
        Month::from(x + rhs)
    }
}

impl Add<u16> for Month {
    type Output = Month;
    fn add(self: Self, rhs: u16) -> Month {
        let x: u16 = self.into();
        Month::from(x + rhs)
    }
}

impl PartialOrd for Month {
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> {
        let lhs: u16 = self.into();
        let rhs: u16 = other.into();
        lhs.partial_cmp(&rhs)
    }
}

impl Ord for Month {
    fn cmp(self: &Self, other: &Self) -> Ordering {
        let lhs: u16 = self.into();
        let rhs: u16 = other.into();
        lhs.cmp(&rhs)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Date {
    pub year: i16,
    pub month: Month,
    pub day_of_month: u8,
}

impl Date {
    pub fn new(y: i16, m: Month, d: u8) -> Date {
        if d < 1 || d > m.non_leap_year_days_in_month() {
            if !(m == Month::Feb && Date::is_year_leap_year(y) && d > 29) {
                panic!("{} {:?} {} is not a valid date", y, m, d);
            }
        }
        Date {
            year: y,
            month: m,
            day_of_month: d,
        }
    }

    pub fn is_year_leap_year(year: i16) -> bool {
        let four = year % 4 == 0;
        let cent = year % 100 == 0;
        let fourcent = year % 400 == 0;

        return four && !(cent && !fourcent);
    }

    pub fn is_leap_year(self: &Self) -> bool {
        Date::is_year_leap_year(self.year)
    }

    pub fn dow_start_of_year(self: &Self) -> DayOfWeek {
        let offset_per_year = 365 % 7;
        // There's a better way to do this without a loop, but I don't
        // feel like working it out right now.
        let leap_year_count: i16 = (1900..self.year)
            .map(|y| Date::new(y, Month::Jan, 1).is_leap_year())
            .map(|x| if x { 1 } else { 0 })
            .sum();
        // 1+ becase 1900 was a Monday
        DayOfWeek::from(1 + ((self.year - 1900) * offset_per_year) + leap_year_count)
    }

    pub fn dow(self: &Self) -> DayOfWeek {
        self.dow_start_of_year() + (self.doy() - 1)
    }

    pub fn doy(self: &Self) -> u16 {
        (self.month.non_leap_year_start_doy_offset() + (self.day_of_month as u16))
            + 1
            + if self.month == Month::Feb && self.is_leap_year() {
                1
            } else {
                0
            }
    }

    pub fn days_in_month(self: &Self) -> u8 {
        self.month.non_leap_year_days_in_month()
            + if self.month == Month::Feb && self.is_leap_year() {
                1
            } else {
                0
            }
    }

    pub fn add_months(self: &Self, i: u16) -> Date {
        let m = &self.month + (i % 12);
        let y = self.year + ((i / 12) as i16);
        let mut dom = self.day_of_month;
        let mut d = Date::new(y, m, 1);
        if dom > d.days_in_month() {
            dom = d.days_in_month();
        }
        d.day_of_month = dom;
        d
    }

    pub fn add_days(self: &Self, i: u16) -> Date {
        let mut i = i;
        let mut d = self.clone();
        while i > 0 {
            let dim = d.days_in_month();
            if ((d.day_of_month as u16) + i) > (dim as u16) {
                i -= (dim - d.day_of_month + 1) as u16;
                d.month = d.month + 1;
                d.day_of_month = 1;
                if d.month == Month::Jan {
                    d.year += 1;
                }
            } else {
                // Since we know that it's less than the days in the month,
                // i can't be bigger than 30.
                d.day_of_month += i as u8;
                break;
            }
        }
        d
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl From<u16> for DayOfWeek {
    fn from(x: u16) -> DayOfWeek {
        match x % 7 {
            0 => DayOfWeek::Sunday,
            1 => DayOfWeek::Monday,
            2 => DayOfWeek::Tuesday,
            3 => DayOfWeek::Wednesday,
            4 => DayOfWeek::Thursday,
            5 => DayOfWeek::Friday,
            6 => DayOfWeek::Saturday,
            _ => panic!("{} isn't a valid day of week number", x),
        }
    }
}
impl From<i16> for DayOfWeek {
    fn from(x: i16) -> DayOfWeek {
        let mut x = x;
        if x < 0 {
            x += 7
        }
        DayOfWeek::from(x as u16)
    }
}
impl Into<u16> for DayOfWeek {
    fn into(self: Self) -> u16 {
        match self {
            DayOfWeek::Sunday => 0,
            DayOfWeek::Monday => 1,
            DayOfWeek::Tuesday => 2,
            DayOfWeek::Wednesday => 3,
            DayOfWeek::Thursday => 4,
            DayOfWeek::Friday => 5,
            DayOfWeek::Saturday => 6,
        }
    }
}
impl Into<u16> for &DayOfWeek {
    fn into(self: Self) -> u16 {
        match self {
            DayOfWeek::Sunday => 0,
            DayOfWeek::Monday => 1,
            DayOfWeek::Tuesday => 2,
            DayOfWeek::Wednesday => 3,
            DayOfWeek::Thursday => 4,
            DayOfWeek::Friday => 5,
            DayOfWeek::Saturday => 6,
        }
    }
}

impl PartialOrd for DayOfWeek {
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> {
        let lhs: u16 = self.into();
        let rhs: u16 = other.into();
        lhs.partial_cmp(&rhs)
    }
}

impl Ord for DayOfWeek {
    fn cmp(self: &Self, other: &Self) -> Ordering {
        let lhs: u16 = self.into();
        let rhs: u16 = other.into();
        lhs.cmp(&rhs)
    }
}

impl Add<u16> for DayOfWeek {
    type Output = DayOfWeek;
    fn add(self: Self, rhs: u16) -> DayOfWeek {
        let x: u16 = self.into();
        DayOfWeek::from(x + rhs)
    }
}

#[cfg(test)]
mod date_test {
    use super::*;
    #[test]
    pub fn leap_years() {
        assert_eq!(Date::new(1900, Month::Jan, 1).is_leap_year(), false);
        assert_eq!(Date::new(1904, Month::Jan, 1).is_leap_year(), true);
        assert_eq!(Date::new(1908, Month::Jan, 1).is_leap_year(), true);
        assert_eq!(Date::new(1951, Month::Jan, 1).is_leap_year(), false);
        assert_eq!(Date::new(1995, Month::Jan, 1).is_leap_year(), false);
        assert_eq!(Date::new(1996, Month::Jan, 1).is_leap_year(), true);
        assert_eq!(Date::new(1997, Month::Jan, 1).is_leap_year(), false);
        assert_eq!(Date::new(2000, Month::Jan, 1).is_leap_year(), true);
    }
    #[test]
    pub fn dow() {
        assert_eq!(Date::new(2024, Month::Aug, 21).dow(), DayOfWeek::Wednesday);
    }
    #[test]
    pub fn doy() {
        assert_eq!(Date::new(2024, Month::Aug, 21).doy(), 234);
    }
    #[test]
    pub fn add_days() {
        assert_eq!(
            Date::new(2024, Month::Aug, 21).add_days(78),
            Date::new(2024, Month::Nov, 7)
        );
        assert_eq!(
            Date::new(2024, Month::Aug, 21).add_days(178),
            Date::new(2025, Month::Feb, 15)
        );
    }
}
