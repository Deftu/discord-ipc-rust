use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    /// string - Guild ID
    pub id: String,
    /// string - Guild name
    pub name: String,
    /// string - Guild icon URL
    pub icon: String,
}
