use chrono::Duration;

use std::collections::HashMap;

use crate::interval::Interval;
use crate::step_group::StepGroup;

#[derive(Debug, PartialEq)]
pub struct Steps {
    map: HashMap<StepGroup, Interval>,
}
macro_rules! step_needed {
    ($map: expr, $var: expr, $key: expr) => {
        if let Some(x) = $var {
            $map.insert($key, x);
        }
    };
}

impl Steps {
    pub fn new(
        brewing: Option<Interval>,
        primary: Option<Interval>,
        diactyl: Option<Interval>,
        secondary: Option<Interval>,
        aginig: Option<Interval>,
        carbonation: Option<Interval>,
    ) -> Self {
        let mut steps = Self {
            map: HashMap::with_capacity(6),
        };
        step_needed!(steps.map, brewing, StepGroup::Brewing);
        step_needed!(steps.map, primary, StepGroup::PrimaryFermentation);
        step_needed!(steps.map, diactyl, StepGroup::DiactylRest);
        step_needed!(steps.map, secondary, StepGroup::SecondaryFermentation);
        step_needed!(steps.map, aginig, StepGroup::Aging);
        step_needed!(steps.map, carbonation, StepGroup::Carbonation);

        steps
    }

    pub fn get(&self, key: &StepGroup) -> Option<&Interval> {
        self.map.get(key)
    }

    pub fn iter(&self) -> StepIterator {
        StepIterator::new(self)
    }

    pub fn range(&self) -> (Duration, Duration) {
        StepIterator::new(self).fold((Duration::hours(0), Duration::hours(0)), |mut acc, step| {
            let (_group, interval) = step;
            let r = interval.range();
            acc.0 = acc.0 + r.0;
            acc.1 = acc.1 + r.1;
            acc
        })
    }
}

pub struct StepIterator<'a> {
    steps: &'a Steps,
    pos: usize,
    order: Vec<StepGroup>,
}

impl<'a> StepIterator<'a> {
    pub fn new(steps: &'a Steps) -> Self {
        Self {
            steps,
            pos: 0,
            order: vec![
                StepGroup::Brewing,
                StepGroup::PrimaryFermentation,
                StepGroup::DiactylRest,
                StepGroup::SecondaryFermentation,
                StepGroup::Aging,
                StepGroup::Carbonation,
            ],
        }
    }
}

impl<'a> std::iter::Iterator for StepIterator<'a> {
    type Item = (StepGroup, Interval);
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.order.len() {
            return None;
        }
        let key = self.order.get(self.pos).unwrap();
        self.pos += 1;
        match self.steps.get(key) {
            Some(interval) => {
                return Some((key.clone(), interval.clone()));
            }
            None => return self.next(),
        }
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;

    pub fn steps() -> Steps {
        Steps::new(
            Some(Interval::Hours(12)),
            Some(Interval::Days(11)),
            None,
            Some(Interval::Weeks(14)),
            Some(Interval::Months(4)),
            Some(Interval::Days(2)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps_new() {
        let steps = mock::steps();
        assert_eq!(
            StepIterator::new(&steps).collect::<Vec<(StepGroup, Interval)>>(),
            vec![
                (StepGroup::Brewing, Interval::Hours(12)),
                (StepGroup::PrimaryFermentation, Interval::Days(11)),
                (StepGroup::SecondaryFermentation, Interval::Weeks(14)),
                (StepGroup::Aging, Interval::Months(4)),
                (StepGroup::Carbonation, Interval::Days(2)),
            ]
        );
        assert_eq!(
            steps.range(),
            (Duration::seconds(18991800), Duration::seconds(21011400))
        )
    }
}
