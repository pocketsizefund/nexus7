use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub enum Region {
    UsWest2,
    UsEast1,
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Region::UsWest2 => write!(f, "us-west-2"),
            Region::UsEast1 => write!(f, "us-east-1"),
        }
    }
}

impl From<Region> for hcl::Expression {
    fn from(region: Region) -> Self {
        hcl::Expression::String(region.to_string())
    }
}
