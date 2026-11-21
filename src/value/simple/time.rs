#[mirl_macros::derive_all]
/// The dd/mm/yyyy
pub struct Date {
    /// Year
    pub year: u16,
    /// Month: 1 to 12
    pub month: u8,
    /// Day: 1 to {28, 29, 30, 31} (based on month/year)
    pub day: u8,
}

#[mirl_macros::derive_all(zerocopy = false)]
/// A specific point in time (optionally offset by timezone)
pub struct Datetime {
    /// [`Date`]
    pub date: Option<Date>,

    /// [`Time`]
    pub time: Option<Time>,

    /// By how many minutes the time region if offset
    /// Minutes: -`1_440..1_440` (-24h..24h)
    pub minute_offset: i16,
}

#[mirl_macros::derive_all(zerocopy = false)]
/// Time that is less that 24 hours
pub struct Time {
    /// Hour: 0 to 23
    pub hour: u8,
    /// Minute: 0 to 59
    pub minute: u8,
    /// Second: 0 to 59
    pub second: u8,
    /// Nanosecond: 0 to `999_999_999`
    pub nanosecond: u32,
}
