use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Reserve {
    pub user: String,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
}
