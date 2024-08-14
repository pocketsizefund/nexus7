use std::fmt;

#[derive(Clone, Debug)]
pub enum AvailabilityZone {
    UsEast1a,
    UsEast1b,
    UsEast1c,
}

impl fmt::Display for AvailabilityZone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl AvailabilityZone {
    pub fn to_string(&self) -> String {
        match self {
            AvailabilityZone::UsEast1a => "us-east-1a".to_string(),
            AvailabilityZone::UsEast1b => "us-east-1b".to_string(),
            AvailabilityZone::UsEast1c => "us-east-1c".to_string(),
        }
    }
}
