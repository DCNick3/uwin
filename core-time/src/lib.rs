use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Weekday};

pub struct SystemTime {
    pub year: u16,
    pub month: u16,
    pub day_of_week: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub millisecond: u16,
}

/// Contains a 64-bit value representing the number of 100-nanosecond intervals since January 1, 1601 (UTC).
pub struct FileTime(pub u64);

pub struct TimeProvider {}

fn weekday_num(weekday: Weekday) -> u16 {
    // see table at https://docs.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-systemtime
    match weekday {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    }
}

fn chrono_datetime_to_systemtime(time: NaiveDateTime) -> SystemTime {
    SystemTime {
        year: time.year().try_into().unwrap(),
        month: time.month().try_into().unwrap(),
        day_of_week: weekday_num(time.weekday()),
        day: time.day().try_into().unwrap(),
        hour: time.hour().try_into().unwrap(),
        minute: time.minute().try_into().unwrap(),
        second: time.second().try_into().unwrap(),
        millisecond: time.timestamp_subsec_millis().try_into().unwrap(),
    }
}

impl TimeProvider {
    // those accept self to later allow to extend TimeProvider to support fake time
    pub fn get_local_time(&self) -> SystemTime {
        let time = chrono::Local::now();

        chrono_datetime_to_systemtime(time.naive_local())
    }

    pub fn get_system_time(&self) -> SystemTime {
        let time = chrono::Utc::now();

        chrono_datetime_to_systemtime(time.naive_local())
    }

    pub fn file_time_to_system_time(&self, file_time: FileTime) -> SystemTime {
        let file_time_origin = NaiveDateTime::new(
            NaiveDate::from_ymd(1601, 1, 1),
            NaiveTime::from_hms(0, 0, 0),
        );

        let timestamp = file_time_origin
            + (Duration::microseconds((file_time.0 / 10) as i64)
                + Duration::nanoseconds(((file_time.0 % 10) * 100) as i64));

        chrono_datetime_to_systemtime(timestamp)
    }
}
