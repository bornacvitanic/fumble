use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Probability(f64);

impl Probability {
    pub fn new(value: f64) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&value) {
            Err(format!("{} is not in the range 0.0 to 1.0", value))
        } else {
            Ok(Probability(value))
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl FromStr for Probability {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: f64 = s
            .parse()
            .map_err(|_| format!("`{}` is not a valid number", s))?;
        Probability::new(value)
    }
}

impl From<Probability> for f64 {
    fn from(prob: Probability) -> Self {
        prob.0
    }
}

impl Default for Probability {
    fn default() -> Self {
        Probability(0.0)
    }
}

impl fmt::Display for Probability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
