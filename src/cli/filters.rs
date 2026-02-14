use std::time::{SystemTime, UNIX_EPOCH};

use clap::Args;

use crate::models::session::Session;

/// Shared filter options for bulk session commands.
#[derive(Args, Clone, Debug, Default)]
pub struct SessionFilters {
    /// Filter by session status (case-insensitive)
    #[arg(long)]
    pub status: Option<String>,

    /// Filter by image ID (exact match)
    #[arg(long)]
    pub image: Option<String>,

    /// Filter by user ID (exact match)
    #[arg(long)]
    pub user: Option<String>,

    /// Filter by hostname (exact match)
    #[arg(long)]
    pub host: Option<String>,

    /// Only sessions created before this datetime (YYYY-MM-DD HH:MM:SS)
    #[arg(long)]
    pub created_before: Option<String>,

    /// Only sessions created after this datetime (YYYY-MM-DD HH:MM:SS)
    #[arg(long)]
    pub created_after: Option<String>,

    /// Only sessions with keepalive_date before this datetime (YYYY-MM-DD HH:MM:SS)
    #[arg(long)]
    pub idle_since: Option<String>,

    /// Only sessions idle for at least this duration (e.g. 30m, 2h, 1d, 1h30m)
    #[arg(long, value_name = "DURATION")]
    pub idle_for: Option<String>,
}

impl SessionFilters {
    /// Returns true when no filters are set.
    pub fn is_empty(&self) -> bool {
        self.status.is_none()
            && self.image.is_none()
            && self.user.is_none()
            && self.host.is_none()
            && self.created_before.is_none()
            && self.created_after.is_none()
            && self.idle_since.is_none()
            && self.idle_for.is_none()
    }

    /// Validate filter inputs before making any API calls.
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref dur) = self.idle_for {
            parse_duration(dur)?;
        }
        Ok(())
    }

    /// Apply all filters to a list of sessions, removing non-matching entries.
    pub fn apply(&self, sessions: &mut Vec<Session>) {
        let idle_threshold = self.idle_for.as_ref().map(|dur| {
            let secs = parse_duration(dur).expect("invalid --idle-for duration");
            format_utc_minus(secs)
        });

        sessions.retain(|s| {
            if let Some(ref status) = self.status {
                let status_lower = status.to_lowercase();
                if s.status
                    .as_ref()
                    .is_none_or(|v| v.to_lowercase() != status_lower)
                {
                    return false;
                }
            }

            if let Some(ref image) = self.image
                && s.image_id.as_deref() != Some(image.as_str())
            {
                return false;
            }

            if let Some(ref user) = self.user
                && s.user_id.as_deref() != Some(user.as_str())
            {
                return false;
            }

            if let Some(ref host) = self.host
                && s.hostname.as_deref() != Some(host.as_str())
            {
                return false;
            }

            if let Some(ref before) = self.created_before
                && s.created_date
                    .as_ref()
                    .is_none_or(|d| d.as_str() >= before.as_str())
            {
                return false;
            }

            if let Some(ref after) = self.created_after
                && s.created_date
                    .as_ref()
                    .is_none_or(|d| d.as_str() <= after.as_str())
            {
                return false;
            }

            if let Some(ref since) = self.idle_since
                && s.keepalive_date
                    .as_ref()
                    .is_none_or(|d| d.as_str() >= since.as_str())
            {
                return false;
            }

            if let Some(ref threshold) = idle_threshold
                && s.keepalive_date
                    .as_ref()
                    .is_none_or(|d| d.as_str() >= threshold.as_str())
            {
                return false;
            }

            true
        });
    }
}

/// Parse a human-friendly duration string into total seconds.
///
/// Supports combinations like `30m`, `2h`, `1d`, `1h30m`, `1d12h`.
fn parse_duration(s: &str) -> Result<u64, String> {
    let mut total: u64 = 0;
    let mut current = String::new();

    for ch in s.chars() {
        match ch {
            '0'..='9' => current.push(ch),
            'd' | 'h' | 'm' => {
                let n: u64 = current
                    .parse()
                    .map_err(|_| format!("invalid number in duration: {s:?}"))?;
                current.clear();
                match ch {
                    'd' => total += n * 86400,
                    'h' => total += n * 3600,
                    'm' => total += n * 60,
                    _ => unreachable!(),
                }
            }
            _ => return Err(format!("unexpected character {ch:?} in duration {s:?}")),
        }
    }

    if !current.is_empty() {
        return Err(format!(
            "trailing digits without unit in duration {s:?} (use d/h/m)"
        ));
    }

    if total == 0 {
        return Err(format!("duration must be greater than zero: {s:?}"));
    }

    Ok(total)
}

/// Return the current UTC time minus `secs` seconds, formatted as `YYYY-MM-DD HH:MM:SS`.
fn format_utc_minus(secs: u64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX epoch");
    let target = now.as_secs().saturating_sub(secs);

    // Convert epoch seconds to date-time components (simplified UTC-only algorithm).
    let (year, month, day, hour, min, sec) = epoch_to_datetime(target);
    format!("{year:04}-{month:02}-{day:02} {hour:02}:{min:02}:{sec:02}")
}

/// Convert seconds since UNIX epoch to (year, month, day, hour, minute, second) in UTC.
fn epoch_to_datetime(epoch: u64) -> (u64, u64, u64, u64, u64, u64) {
    let sec = epoch % 60;
    let min = (epoch / 60) % 60;
    let hour = (epoch / 3600) % 24;
    let mut days = epoch / 86400;

    // Civil date from day count (algorithm from Howard Hinnant)
    days += 719_468;
    let era = days / 146_097;
    let doe = days - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    (y, m, d, hour, min, sec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_duration_minutes() {
        assert_eq!(parse_duration("30m").unwrap(), 1800);
    }

    #[test]
    fn parse_duration_hours() {
        assert_eq!(parse_duration("2h").unwrap(), 7200);
    }

    #[test]
    fn parse_duration_days() {
        assert_eq!(parse_duration("1d").unwrap(), 86400);
    }

    #[test]
    fn parse_duration_combined() {
        assert_eq!(parse_duration("1h30m").unwrap(), 5400);
        assert_eq!(parse_duration("1d12h").unwrap(), 129600);
    }

    #[test]
    fn parse_duration_rejects_empty() {
        assert!(parse_duration("").is_err());
    }

    #[test]
    fn parse_duration_rejects_no_unit() {
        assert!(parse_duration("30").is_err());
    }

    #[test]
    fn epoch_to_datetime_unix_epoch() {
        assert_eq!(epoch_to_datetime(0), (1970, 1, 1, 0, 0, 0));
    }

    #[test]
    fn epoch_to_datetime_known_date() {
        // 2024-01-15 12:30:45 UTC = 1705321845
        assert_eq!(epoch_to_datetime(1705321845), (2024, 1, 15, 12, 30, 45));
    }

    // --- apply() tests ---

    fn make_session(overrides: impl FnOnce(&mut Session)) -> Session {
        let mut s = Session {
            kasm_id: "test-id".into(),
            status: Some("running".into()),
            image_id: Some("img-1".into()),
            user_id: Some("user-1".into()),
            hostname: Some("host-1".into()),
            created_date: Some("2025-01-15 12:00:00".into()),
            keepalive_date: Some("2025-06-01 10:00:00".into()),
            ..Default::default()
        };
        overrides(&mut s);
        s
    }

    #[test]
    fn filter_by_status() {
        let filters = SessionFilters {
            status: Some("Running".into()),
            ..Default::default()
        };
        let mut sessions = vec![
            make_session(|s| s.status = Some("running".into())),
            make_session(|s| s.status = Some("stopped".into())),
            make_session(|s| s.status = Some("RUNNING".into())),
        ];
        filters.apply(&mut sessions);
        assert_eq!(sessions.len(), 2);
        assert!(
            sessions
                .iter()
                .all(|s| s.status.as_ref().unwrap().to_lowercase() == "running")
        );
    }

    #[test]
    fn filter_by_image() {
        let filters = SessionFilters {
            image: Some("img-1".into()),
            ..Default::default()
        };
        let mut sessions = vec![
            make_session(|s| s.image_id = Some("img-1".into())),
            make_session(|s| s.image_id = Some("img-2".into())),
        ];
        filters.apply(&mut sessions);
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].image_id.as_deref(), Some("img-1"));
    }

    #[test]
    fn filter_by_user() {
        let filters = SessionFilters {
            user: Some("user-1".into()),
            ..Default::default()
        };
        let mut sessions = vec![
            make_session(|s| s.user_id = Some("user-1".into())),
            make_session(|s| s.user_id = Some("user-2".into())),
        ];
        filters.apply(&mut sessions);
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].user_id.as_deref(), Some("user-1"));
    }

    #[test]
    fn filter_by_host() {
        let filters = SessionFilters {
            host: Some("host-1".into()),
            ..Default::default()
        };
        let mut sessions = vec![
            make_session(|s| s.hostname = Some("host-1".into())),
            make_session(|s| s.hostname = Some("host-2".into())),
        ];
        filters.apply(&mut sessions);
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].hostname.as_deref(), Some("host-1"));
    }

    #[test]
    fn filter_by_created_before() {
        let filters = SessionFilters {
            created_before: Some("2025-02-01 00:00:00".into()),
            ..Default::default()
        };
        let mut sessions = vec![
            make_session(|s| s.created_date = Some("2025-01-15 12:00:00".into())),
            make_session(|s| s.created_date = Some("2025-03-01 12:00:00".into())),
        ];
        filters.apply(&mut sessions);
        assert_eq!(sessions.len(), 1);
        assert_eq!(
            sessions[0].created_date.as_deref(),
            Some("2025-01-15 12:00:00")
        );
    }

    #[test]
    fn filter_by_created_after() {
        let filters = SessionFilters {
            created_after: Some("2025-02-01 00:00:00".into()),
            ..Default::default()
        };
        let mut sessions = vec![
            make_session(|s| s.created_date = Some("2025-01-15 12:00:00".into())),
            make_session(|s| s.created_date = Some("2025-03-01 12:00:00".into())),
        ];
        filters.apply(&mut sessions);
        assert_eq!(sessions.len(), 1);
        assert_eq!(
            sessions[0].created_date.as_deref(),
            Some("2025-03-01 12:00:00")
        );
    }

    #[test]
    fn filter_by_idle_since() {
        let filters = SessionFilters {
            idle_since: Some("2025-06-01 12:00:00".into()),
            ..Default::default()
        };
        let mut sessions = vec![
            make_session(|s| s.keepalive_date = Some("2025-06-01 10:00:00".into())),
            make_session(|s| s.keepalive_date = Some("2025-06-01 14:00:00".into())),
        ];
        filters.apply(&mut sessions);
        assert_eq!(sessions.len(), 1);
        assert_eq!(
            sessions[0].keepalive_date.as_deref(),
            Some("2025-06-01 10:00:00")
        );
    }

    #[test]
    fn filter_combined() {
        let filters = SessionFilters {
            status: Some("running".into()),
            image: Some("img-1".into()),
            ..Default::default()
        };
        let mut sessions = vec![
            make_session(|s| {
                s.status = Some("running".into());
                s.image_id = Some("img-1".into());
            }),
            make_session(|s| {
                s.status = Some("running".into());
                s.image_id = Some("img-2".into());
            }),
            make_session(|s| {
                s.status = Some("stopped".into());
                s.image_id = Some("img-1".into());
            }),
        ];
        filters.apply(&mut sessions);
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].status.as_deref(), Some("running"));
        assert_eq!(sessions[0].image_id.as_deref(), Some("img-1"));
    }

    #[test]
    fn filter_no_filters_retains_all() {
        let filters = SessionFilters::default();
        let mut sessions = vec![
            make_session(|_| {}),
            make_session(|_| {}),
            make_session(|_| {}),
        ];
        filters.apply(&mut sessions);
        assert_eq!(sessions.len(), 3);
    }

    #[test]
    fn parse_duration_rejects_invalid_unit() {
        assert!(parse_duration("30s").is_err());
    }

    #[test]
    fn parse_duration_rejects_mixed_invalid() {
        assert!(parse_duration("1h30x").is_err());
    }

    // --- validate() tests ---

    #[test]
    fn validate_returns_error_on_invalid_idle_for() {
        let filters = SessionFilters {
            idle_for: Some("30s".into()),
            ..Default::default()
        };
        assert!(filters.validate().is_err());
    }

    #[test]
    fn validate_succeeds_with_valid_idle_for() {
        let filters = SessionFilters {
            idle_for: Some("1h30m".into()),
            ..Default::default()
        };
        assert!(filters.validate().is_ok());
    }

    #[test]
    fn validate_succeeds_when_no_filters() {
        assert!(SessionFilters::default().validate().is_ok());
    }

    // --- is_empty() tests ---

    #[test]
    fn is_empty_when_default() {
        assert!(SessionFilters::default().is_empty());
    }

    #[test]
    fn is_empty_false_when_any_set() {
        assert!(
            !SessionFilters {
                status: Some("running".into()),
                ..Default::default()
            }
            .is_empty()
        );

        assert!(
            !SessionFilters {
                image: Some("img-1".into()),
                ..Default::default()
            }
            .is_empty()
        );

        assert!(
            !SessionFilters {
                user: Some("user-1".into()),
                ..Default::default()
            }
            .is_empty()
        );

        assert!(
            !SessionFilters {
                host: Some("host-1".into()),
                ..Default::default()
            }
            .is_empty()
        );

        assert!(
            !SessionFilters {
                created_before: Some("2025-01-01 00:00:00".into()),
                ..Default::default()
            }
            .is_empty()
        );

        assert!(
            !SessionFilters {
                created_after: Some("2025-01-01 00:00:00".into()),
                ..Default::default()
            }
            .is_empty()
        );

        assert!(
            !SessionFilters {
                idle_since: Some("2025-01-01 00:00:00".into()),
                ..Default::default()
            }
            .is_empty()
        );

        assert!(
            !SessionFilters {
                idle_for: Some("1h".into()),
                ..Default::default()
            }
            .is_empty()
        );
    }
}
