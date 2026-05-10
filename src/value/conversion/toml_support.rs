use crate::value::{Date, Datetime, Time};

impl From<toml::value::Date> for Date {
    fn from(value: toml::value::Date) -> Self {
        Self {
            year: value.year,
            month: value.month,
            day: value.day,
        }
    }
}
impl From<toml::value::Time> for Time {
    fn from(value: toml::value::Time) -> Self {
        Self {
            hour: value.hour,
            minute: value.minute,
            second: value.second.unwrap_or_default(),
            nanosecond: value.nanosecond.unwrap_or_default(),
        }
    }
}
impl From<toml::value::Datetime> for Datetime {
    fn from(value: toml::value::Datetime) -> Self {
        Self {
            date: value.date.map(Into::into),
            time: value.time.map(Into::into),
            minute_offset: value.offset.map_or(0, |x| match x {
                toml::value::Offset::Z => 0,
                toml::value::Offset::Custom {
                    minutes,
                } => minutes,
            }),
        }
    }
}
