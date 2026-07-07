//! Shared numeric keys for date-based lookups.

pub(crate) const fn month_day_key(month: i32, day: i32) -> i32 {
    month * 100 + day
}

pub(crate) const fn month_weekday_key(month: i32, week_index: i32, week: i32) -> i32 {
    month * 100 + week_index * 10 + week
}

pub(crate) fn parse_month_day_key(key: &str) -> Option<i32> {
    let mut parts = key.rsplitn(2, '-');
    let day = parts.next()?.parse::<i32>().ok()?;
    let month = parts.next()?.parse::<i32>().ok()?;
    Some(month_day_key(month, day))
}

pub(crate) fn parse_month_weekday_key(key: &str) -> Option<i32> {
    let mut parts = key.rsplitn(3, '-');
    let week = parts.next()?.parse::<i32>().ok()?;
    let week_index = parts.next()?.parse::<i32>().ok()?;
    let month = parts.next()?.parse::<i32>().ok()?;
    Some(month_weekday_key(month, week_index, week))
}
