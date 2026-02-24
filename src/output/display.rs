use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Returns the first 8 characters of a UUID-like string (like Docker/git short IDs).
/// If the input is shorter than 8 characters, returns the whole string.
pub fn short_id(id: &str) -> &str {
    if id.len() <= 8 { id } else { &id[..8] }
}

/// Parses a `"YYYY-MM-DD HH:MM:SS"` datetime string and returns a human-friendly
/// relative age like `"2h ago"`, `"3d ago"`, etc.
/// Falls back to returning the original string if parsing fails.
pub fn relative_age(datetime: &str) -> String {
    let Some(epoch) = parse_datetime_to_epoch(datetime) else {
        return datetime.to_string();
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs();

    let diff = now.saturating_sub(epoch);
    format_duration_ago(diff)
}

/// Convert a `"YYYY-MM-DD HH:MM:SS"` UTC datetime string to seconds since the UNIX epoch.
/// Returns `None` if the string cannot be parsed.
fn parse_datetime_to_epoch(s: &str) -> Option<u64> {
    let b = s.as_bytes();
    if b.len() != 19 {
        return None;
    }
    // All expected characters are ASCII, so byte == char for valid inputs.
    // If a byte in a digit position is not ASCII, from_utf8/parse will return Err → None.
    let year: u64 = std::str::from_utf8(&b[0..4]).ok()?.parse().ok()?;
    let month: u64 = std::str::from_utf8(&b[5..7]).ok()?.parse().ok()?;
    let day: u64 = std::str::from_utf8(&b[8..10]).ok()?.parse().ok()?;
    let hour: u64 = std::str::from_utf8(&b[11..13]).ok()?.parse().ok()?;
    let min: u64 = std::str::from_utf8(&b[14..16]).ok()?.parse().ok()?;
    let sec: u64 = std::str::from_utf8(&b[17..19]).ok()?.parse().ok()?;

    if !(1..=12).contains(&month) || !(1..=31).contains(&day) || hour > 23 || min > 59 || sec > 59 {
        return None;
    }

    // Inverse of the Hinnant algorithm: civil date → days since UNIX epoch.
    // Shift so March = month 0 (avoids leap-year boundary at year end).
    let (y, m) = if month <= 2 {
        (year.checked_sub(1)?, month + 9)
    } else {
        (year, month - 3)
    };

    let era = y / 400;
    let yoe = y - era * 400;
    let doy = (153 * m + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days_from_ref = era.checked_mul(146_097)?.checked_add(doe)?;
    // 719_468 = days from Hinnant's reference (Mar 1, year 0) to 1970-01-01
    let days = days_from_ref.checked_sub(719_468)?;

    let epoch = days
        .checked_mul(86_400)?
        .checked_add(hour * 3600)?
        .checked_add(min * 60)?
        .checked_add(sec)?;

    Some(epoch)
}

/// Format a duration in seconds as a human-friendly "X unit ago" string.
fn format_duration_ago(secs: u64) -> String {
    if secs < 60 {
        format!("{}s ago", secs)
    } else if secs < 3600 {
        format!("{}m ago", secs / 60)
    } else if secs < 86400 {
        format!("{}h ago", secs / 3600)
    } else {
        format!("{}d ago", secs / 86400)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- short_id ---

    #[test]
    fn short_id_truncates_to_8_chars() {
        assert_eq!(short_id("550e8400-e29b-41d4"), "550e8400");
    }

    #[test]
    fn short_id_returns_whole_string_when_short() {
        assert_eq!(short_id("abc"), "abc");
    }

    #[test]
    fn short_id_returns_exactly_8_chars_unchanged() {
        assert_eq!(short_id("12345678"), "12345678");
    }

    #[test]
    fn short_id_empty_string() {
        assert_eq!(short_id(""), "");
    }

    // --- parse_datetime_to_epoch ---

    #[test]
    fn parse_known_date() {
        // 2024-01-15 12:30:45 UTC = 1705321845
        assert_eq!(
            parse_datetime_to_epoch("2024-01-15 12:30:45"),
            Some(1705321845)
        );
    }

    #[test]
    fn parse_unix_epoch_origin() {
        assert_eq!(parse_datetime_to_epoch("1970-01-01 00:00:00"), Some(0));
    }

    #[test]
    fn parse_returns_none_on_bad_format() {
        assert_eq!(parse_datetime_to_epoch("not a date"), None);
        assert_eq!(parse_datetime_to_epoch("2024-13-01 00:00:00"), None); // invalid month
        assert_eq!(parse_datetime_to_epoch("2024-01-01 25:00:00"), None); // invalid hour
    }

    // --- format_duration_ago ---

    #[test]
    fn format_seconds() {
        assert_eq!(format_duration_ago(45), "45s ago");
    }

    #[test]
    fn format_minutes() {
        assert_eq!(format_duration_ago(90), "1m ago");
        assert_eq!(format_duration_ago(3599), "59m ago");
    }

    #[test]
    fn format_hours() {
        assert_eq!(format_duration_ago(3600), "1h ago");
        assert_eq!(format_duration_ago(5400), "1h ago"); // 90 minutes = 1h
        assert_eq!(format_duration_ago(86399), "23h ago");
    }

    #[test]
    fn format_days() {
        assert_eq!(format_duration_ago(86400), "1d ago");
        assert_eq!(format_duration_ago(90000), "1d ago"); // 25h = 1d
    }

    // --- relative_age (fallback on bad input) ---

    #[test]
    fn relative_age_falls_back_on_bad_input() {
        let bad = "not-a-datetime";
        assert_eq!(relative_age(bad), bad);
    }
}
