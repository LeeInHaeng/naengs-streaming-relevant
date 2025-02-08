use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OauthCallbackParam {
    pub code: String,
    pub state: String,
}