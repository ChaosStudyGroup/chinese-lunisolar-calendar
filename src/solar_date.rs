use super::{LunisolarError, SolarYear, SolarMonth, SolarDay, LunarDate, NEW_YEAR_DIFFERENCE, MIN_YEAR_IN_SOLAR_CALENDAR};

use std::fmt::{self, Display, Formatter};

use chrono::prelude::*;

use chrono::NaiveDate;

/// 西曆年月日。
#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
pub struct SolarDate {
    solar_year: SolarYear,
    solar_month: SolarMonth,
    solar_day: SolarDay,
}

impl SolarDate {
    /// 將無時區的 `Chrono` 年月日實體轉成 `SolarDate` 實體。
    pub fn from_naive_date(naive_date: NaiveDate) -> Result<SolarDate, LunisolarError> {
        let year = naive_date.year();

        if year < 0 || year > u16::max_value() as i32 {
            Err(LunisolarError::OutOfSolarRange)
        } else {
            let solar_year = SolarYear::from_u16(year as u16);
            let solar_month = SolarMonth::from_u8(naive_date.month() as u8).unwrap();
            let solar_day = SolarDay::from_u8(naive_date.day() as u8).unwrap();

            Ok(SolarDate {
                solar_year,
                solar_month,
                solar_day,
            })
        }
    }

    /// 將有時區的 `Chrono` 年月日實體，依UTC時區轉成 `SolarDate` 實體。
    pub fn from_date<Tz: TimeZone>(date: Date<Tz>) -> Result<SolarDate, LunisolarError> {
        let naive_date = date.naive_utc();

        Self::from_naive_date(naive_date)
    }

    /// 將 `SolarDate` 實體轉成無時區的 `Chrono` 年月日實體。
    pub fn to_naive_date(&self) -> NaiveDate {
        NaiveDate::from_ymd(self.solar_year.to_u16() as i32, self.solar_month.to_u8() as u32, self.solar_day.to_u8() as u32)
    }

    /// 將 `SolarDate` 實體轉成UTC時區的 `Chrono` 年月日實體。
    pub fn to_date_utc(&self) -> Date<Utc> {
        let naive_date = self.to_naive_date();

        Date::from_utc(naive_date, Utc)
    }

    /// 利用西曆的年月日來產生 `SolarDate` 實體。
    pub fn from_solar_year_month_day<Y: Into<SolarYear>>(solar_year: Y, solar_month: SolarMonth, solar_day: SolarDay) -> Result<SolarDate, LunisolarError> {
        let solar_year = solar_year.into();

        let days = solar_month.get_total_days(solar_year);

        let day = solar_day.to_u8();

        if day > days {
            Err(LunisolarError::IncorrectSolarDay)
        } else {
            Ok(SolarDate {
                solar_year,
                solar_month,
                solar_day,
            })
        }
    }

    /// 利用西曆的年月日來產生 `SolarDate` 實體。
    pub fn from_ymd(year: u16, month: u8, day: u8) -> Result<SolarDate, LunisolarError> {
        let solar_year = SolarYear::from_u16(year);

        let solar_month = match SolarMonth::from_u8(month) {
            Some(solar_month) => solar_month,
            None => return Err(LunisolarError::IncorrectSolarMonth)
        };

        let solar_day = match SolarDay::from_u8(day) {
            Some(solar_day) => solar_day,
            None => return Err(LunisolarError::IncorrectSolarMonth)
        };

        Self::from_solar_year_month_day(solar_year, solar_month, solar_day)
    }


    /// 利用農曆年月日來產生 `SolarDate` 實體。
    pub fn from_lunar_date(lunar_date: LunarDate) -> SolarDate {
        let n = lunar_date.the_n_day_in_this_year();

        let solar_year = lunar_date.get_solar_year();

        let sy = solar_year.to_u16();

        let lunisolar_year = lunar_date.get_lunisolar_year();

        let ly = lunisolar_year.to_u16();

        let mut days_diff = if sy == ly {
            n - 1 + NEW_YEAR_DIFFERENCE[(ly - MIN_YEAR_IN_SOLAR_CALENDAR) as usize] as u16 // 天數差距為該農曆日期與對應西曆年新年的天數差距。其實就是轉換成西曆日期後，西曆日期與新年的距離。(舉例，農曆2012-01-03，為第3天，和農曆新年差了2天。加上西曆農曆偏差52天。因此天數差距為54)
        } else {
            n - 1 - (lunisolar_year.to_solar_year().get_total_days() - NEW_YEAR_DIFFERENCE[(ly - MIN_YEAR_IN_SOLAR_CALENDAR) as usize] as u16)
        };

        let mut month = 1;

        let mut solar_month = unsafe { SolarMonth::from_u8_unsafe(month) };

        let mut month_days = solar_month.get_total_days(solar_year) as u16;

        while days_diff >= month_days {
            days_diff -= month_days;

            if month == 12 {
                break;
            }

            month += 1;

            solar_month = unsafe { SolarMonth::from_u8_unsafe(month) };

            month_days = solar_month.get_total_days(solar_year) as u16;
        }

        SolarDate {
            solar_year,
            solar_month,
            solar_day: unsafe { SolarDay::from_u8_unsafe(days_diff as u8 + 1) },
        }
    }


    /// 轉成農曆年月日。
    pub fn to_lunar_date(&self) -> Result<LunarDate, LunisolarError> {
        LunarDate::from_solar_date(*self)
    }

    /// 以目前的年月日來產生 `SolarDate` 實體。
    pub fn now() -> Result<SolarDate, LunisolarError> {
        Self::from_date(Utc::now().date())
    }

    pub fn from_str<S: AsRef<str>>(s: S) -> Result<SolarDate, LunisolarError> {
        let s = s.as_ref();

        let year_index = {
            match s.find("年") {
                Some(index) => index,
                None => match s.find("　") {
                    Some(index) => index,
                    None => return Err(LunisolarError::IncorrectSolarYear)
                }
            }
        };

        let year_str = s[..year_index].trim();

        let solar_year = match SolarYear::from_str(year_str) {
            Some(solar_year) => solar_year,
            None => return Err(LunisolarError::IncorrectSolarYear)
        };

        let s = &s[year_index + 3..];

        let month_index = {
            match s.find("月") {
                Some(index) => index,
                None => match s.find("　") {
                    Some(index) => index,
                    None => return Err(LunisolarError::IncorrectSolarMonth)
                }
            }
        };

        let month_str = s[..month_index + 3].trim();

        let solar_month = match SolarMonth::from_str(month_str) {
            Some(solar_month) => solar_month,
            None => return Err(LunisolarError::IncorrectSolarMonth)
        };

        let mut day_str = s[month_index + 3..].trim();

        if day_str.ends_with("日") {
            day_str = &day_str[..day_str.len() - 3];
        }

        let solar_day = match SolarDay::from_str(day_str) {
            Some(solar_day) => solar_day,
            None => return Err(LunisolarError::IncorrectSolarDay)
        };

        Self::from_solar_year_month_day(solar_year, solar_month, solar_day)
    }

    /// 取得 `SolarDate` 實體所代表的中文西曆年月日字串。
    pub fn to_chinese_string(&self) -> String {
        let mut s = String::new();

        self.write_to_chinese_string(&mut s);

        s
    }

    /// 取得 `SolarDate` 實體所代表的中文西曆年月日字串。
    pub fn write_to_chinese_string(&self, s: &mut String) {
        s.reserve(36);

        self.solar_year.write_to_chinese_string(s);
        s.push_str("年");
        s.push_str(self.solar_month.to_str());
        s.push_str(self.solar_day.to_str());
        s.push_str("日");
    }

    /// 取得西曆年。
    pub fn get_solar_year(&self) -> SolarYear {
        self.solar_year
    }

    /// 取得西曆月。
    pub fn get_solar_month(&self) -> SolarMonth {
        self.solar_month
    }

    /// 取得西曆日。
    pub fn get_solar_day(&self) -> SolarDay {
        self.solar_day
    }

    /// 計算此西曆年月日是該西曆年的第幾天。舉例：2013-01-04，就是第四天。
    pub fn the_n_day_in_this_year(&self) -> u16 {
        let mut n = 0;

        let solar_year = self.solar_year;

        let month = self.solar_month.to_u8();

        for i in 1..month {
            let solar_month = unsafe { SolarMonth::from_u8_unsafe(i) };
            n += solar_month.get_total_days(solar_year) as u16;
        }

        n += self.solar_day.to_u8() as u16;

        n
    }
}

impl Display for SolarDate {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.to_chinese_string())
    }
}