use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use uuid::Uuid;

#[derive(
    Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    sqlx::Type,
)]
#[sqlx(transparent)]
pub struct ServiceId(Uuid);

impl ServiceId {
    pub fn new(id: Uuid) -> Self {
        ServiceId(id)
    }
}

impl FromStr for ServiceId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Uuid::parse_str(s)?;
        Ok(ServiceId::new(id))
    }
}

impl Display for ServiceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{}", self.0.to_hyphenated()))
    }
}
