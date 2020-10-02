use chrono::Duration;

#[derive(Debug, PartialEq)]
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

    pub fn range(&self) -> (Duration, Duration) {
        match self {
            Interval::Hours(x) => {
                let delta = Duration::minutes(30);
                let d = Duration::hours(*x as i64);
                (d - delta, d + delta)
            }
            Interval::Days(x) => {
                let delta = Duration::hours(8);
                let d = Duration::days(*x as i64);
                (d - delta, d + delta)
            }
            Interval::Weeks(x) => {
                let delta = Duration::days(4);
                let d = Duration::days(*x as i64 * 7);
                (d - delta, d + delta)
            }
            Interval::Months(x) => {
                let delta = Duration::days(7);
                let d = Duration::days(*x as i64 * 30);
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
