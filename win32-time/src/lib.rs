use core_time::{FileTime, SystemTime, TimeProvider};
use win32::Win32::Foundation::{FILETIME, SYSTEMTIME};

fn system_time_to_win32(time: SystemTime) -> SYSTEMTIME {
    SYSTEMTIME {
        wYear: time.year,
        wMonth: time.month,
        wDayOfWeek: time.day_of_week,
        wDay: time.day,
        wHour: time.hour,
        wMinute: time.minute,
        wSecond: time.second,
        wMilliseconds: time.millisecond,
    }
}

fn file_time_from_win32(time: FILETIME) -> FileTime {
    FileTime(time.dwLowDateTime as u64 | ((time.dwHighDateTime as u64) << 32))
}

pub struct Win32TimeProvider {
    pub inner: TimeProvider,
}

impl Win32TimeProvider {
    pub fn get_local_time(&self) -> SYSTEMTIME {
        system_time_to_win32(self.inner.get_local_time())
    }
    pub fn get_system_time(&self) -> SYSTEMTIME {
        system_time_to_win32(self.inner.get_system_time())
    }

    pub fn file_time_to_system_time(&self, file_time: FILETIME) -> SYSTEMTIME {
        system_time_to_win32(
            self.inner
                .file_time_to_system_time(file_time_from_win32(file_time)),
        )
    }
}
