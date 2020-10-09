use chrono::Duration;

const DELTA_FOR_HOURS_IN_MINUTES: i64 = 30;
const DELTA_FOR_DAYS_IN_HOURS: i64 = 8;
const DELTA_FOR_WEEKS_IN_DAYS: i64 = 4;
const DELTA_FOR_MONTHS_IN_DAYS: i64 = 7;

#[derive(Clone, Debug, PartialEq)]
pub enum Interval {
    Hours(u8),
    Days(u8),
    Weeks(u8),
    Months(u8),
}

impl Interval {
    pub fn lookup(&self) -> String {
        match self {
            Interval::Hours(x) => format!("{}h", x),
            Interval::Days(x) => format!("{}d", x),
            Interval::Weeks(x) => format!("{}w", x),
            Interval::Months(x) => format!("{}m", x),
        }
    }
    pub fn duration(&self) -> Duration {
        match self {
            Interval::Hours(x) => Duration::hours(*x as i64),
            Interval::Days(x) => Duration::days(*x as i64),
            Interval::Weeks(x) => Duration::days(*x as i64 * 7),
            Interval::Months(x) => Duration::days(*x as i64 * 30),
        }
    }

    pub fn range(&self) -> (Duration, Duration) {
        let d = self.duration();
        match self {
            Interval::Hours(_x) => {
                let delta = Duration::minutes(DELTA_FOR_HOURS_IN_MINUTES);
                (d - delta, d + delta)
            }
            Interval::Days(_x) => {
                let delta = Duration::hours(DELTA_FOR_DAYS_IN_HOURS);
                (d - delta, d + delta)
            }
            Interval::Weeks(_x) => {
                let delta = Duration::days(DELTA_FOR_WEEKS_IN_DAYS);
                (d - delta, d + delta)
            }
            Interval::Months(_x) => {
                let delta = Duration::days(DELTA_FOR_MONTHS_IN_DAYS);
                (d - delta, d + delta)
            }
        }
    }
}

impl std::str::FromStr for Interval {
    type Err = ();

    fn from_str(s: &str) -> Result<Interval, ()> {
        if s.len() > 1 {
            if let Ok(duration) = s[0..s.len() - 1].parse() {
                match s.chars().last().unwrap() {
                    'h' => return Ok(Interval::Hours(duration)),
                    'd' => return Ok(Interval::Days(duration)),
                    'w' => return Ok(Interval::Weeks(duration)),
                    'm' => return Ok(Interval::Months(duration)),
                    _ => {}
                }
            }
        }
        Err(())
    }
}

#[cfg(test)]
mod mock {
    use super::*;
    pub fn hours() -> Interval {
        Interval::Hours(5)
    }

    pub fn days() -> Interval {
        Interval::Days(5)
    }

    pub fn weeks() -> Interval {
        Interval::Weeks(5)
    }

    pub fn months() -> Interval {
        Interval::Months(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_lookup() {
        assert_eq!(Interval::Hours(8).lookup(), "8h");
        assert_eq!(Interval::Days(8).lookup(), "8d");
        assert_eq!(Interval::Weeks(8).lookup(), "8w");
        assert_eq!(Interval::Months(8).lookup(), "8m");
    }

    #[test]
    fn test_interval_parse() {
        assert_eq!("8h".parse(), Ok(Interval::Hours(8)));
        assert_eq!("8d".parse(), Ok(Interval::Days(8)));
        assert_eq!("8w".parse(), Ok(Interval::Weeks(8)));
        assert_eq!("8m".parse(), Ok(Interval::Months(8)));
    }

    #[test]
    fn test_interval_duration() {
        assert_eq!(Interval::Hours(8).duration(), Duration::seconds(8 * 3600));
        assert_eq!(
            Interval::Days(8).duration(),
            Duration::seconds(8 * 24 * 3600)
        );
        assert_eq!(
            Interval::Weeks(8).duration(),
            Duration::seconds(8 * 7 * 24 * 3600)
        );
        assert_eq!(
            Interval::Months(8).duration(),
            Duration::seconds(8 * 30 * 24 * 3600)
        );
    }

    #[test]
    fn test_interval_range() {
        assert_eq!(
            mock::hours().range(),
            (
                Duration::minutes((4.5 * 60.0) as i64),
                Duration::minutes((5.5 * 60.0) as i64)
            )
        );
        assert_eq!(
            mock::days().range(),
            (
                Duration::hours((24 * 5 - 8) as i64),
                Duration::hours((24 * 5 + 8) as i64)
            )
        );
        assert_eq!(
            mock::weeks().range(),
            (Duration::days(5 * 7 - 4), Duration::days(5 * 7 + 4))
        );
        assert_eq!(
            mock::months().range(),
            (Duration::days(5 * 30 - 7), Duration::days(5 * 30 + 7))
        );
    }
}
