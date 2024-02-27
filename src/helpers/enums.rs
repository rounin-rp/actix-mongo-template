use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UserStatus {
    Active,
    Inactive,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OAuthType {
    Google,
    Facebook,
    None,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum JwtTokenType {
    Access,
    Refresh,
}
