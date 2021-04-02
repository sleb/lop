use crate::error::{LopError, LopResult};
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Range {
    From(usize),
    To(usize),
    Inclusive(usize, usize),
    Singleton(usize),
}

impl FromStr for Range {
    type Err = LopError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bounds = s
            .split('-')
            .map(|bound| match bound {
                "" => Ok(None),
                s => {
                    let n = s.parse()?;
                    if n < 1 {
                        Err(LopError::MalformedRangSpec)
                    } else {
                        Ok(Some(n))
                    }
                }
            })
            .collect::<LopResult<Vec<_>>>()?;

        match *bounds.as_slice() {
            [Some(n)] => Ok(Range::Singleton(n)),
            [Some(s), Some(e)] => {
                if s > e {
                    Err(LopError::MalformedRangSpec)
                } else {
                    Ok(Range::Inclusive(s, e))
                }
            }
            [Some(s), None] => Ok(Range::From(s)),
            [None, Some(e)] => Ok(Range::To(e)),
            _ => Err(LopError::MalformedRangSpec),
        }
    }
}

impl Range {
    fn contains(&self, n: usize) -> bool {
        n != 0 // Ranges are defined to start at 1
            && match *self {
                Range::From(from) => (from..).contains(&n),
                Range::To(to) => (1..=to).contains(&n),
                Range::Inclusive(from, to) => (from..=to).contains(&n),
                Range::Singleton(s) => s == n,
            }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct RangeList {
    ranges: Vec<Range>,
}

impl RangeList {
    pub fn from<I: IntoIterator<Item = Range>>(iter: I) -> RangeList {
        RangeList {
            ranges: Vec::from_iter(iter),
        }
    }

    pub fn from_spec<T: AsRef<str>>(spec: T) -> LopResult<RangeList> {
        // "-5,10,14-17,20-"
        let ranges = spec
            .as_ref()
            .split(|c| c == ',' || c == ' ') // e.g. ["-5", "10", "14-17", "20-"]
            // [[None, Some(5)], [Some(10)], [Some(14), Some(17)], [Some(20), None]]
            .map(|range| range.parse())
            .collect::<LopResult<Vec<_>>>()?;
        Ok(RangeList::from(ranges))
    }

    pub fn contains(&self, n: usize) -> bool {
        self.ranges.iter().any(|range| range.contains(n))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contains() {
        let r = RangeList::from(vec![
            Range::From(100),
            Range::Inclusive(50, 60),
            Range::Singleton(40),
            Range::To(10),
        ]);

        for n in 0..1000 {
            match n {
                1..=10 | 40..=40 | 50..=60 | 100..=1000 => {
                    assert!(r.contains(n), "should contain {}", n)
                }
                _ => assert!(!r.contains(n), "shouldn't contain {}", n),
            }
        }
    }

    #[test]
    fn from_spec() {
        let r1 = RangeList::from(vec![Range::Singleton(1)]);
        let r2 = RangeList::from_spec("1");
        assert_eq!(r1, r2.unwrap());

        let r1 = RangeList::from(vec![
            Range::To(10),
            Range::Singleton(40),
            Range::Inclusive(50, 60),
            Range::From(100),
        ]);

        let r2 = RangeList::from_spec("-10,40,50-60,100-");

        assert_eq!(r1, r2.unwrap());
    }

    #[test]
    fn from_spec_bad() {
        assert!(RangeList::from_spec("b").is_err());
        assert!(RangeList::from_spec("4-5-6").is_err());
    }
}
