use chrono::{DateTime, Datelike, TimeZone, Timelike, Weekday};

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

fn chrono_datetime_to_systemtime<Tz: TimeZone>(time: DateTime<Tz>) -> SystemTime {
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

        chrono_datetime_to_systemtime(time)
    }

    pub fn get_system_time(&self) -> SystemTime {
        let time = chrono::Utc::now();

        chrono_datetime_to_systemtime(time)
    }
}
