use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use regex::Regex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserState {
    Active,
    NotVerify,
    Banned,
    Deleted,
}

pub type UserId = Uuid;
pub const FIRSTNAME_LENGTH_RANGE: (usize, usize) = (4, 64);
pub const LASTNAME_LENGTH_RANGE: (usize, usize) = (4, 64);
pub const USERNAME_LENGTH_RANGE: (usize, usize) = (4, 32);
pub const EMAIL_LENGTH_RANGE: (usize, usize) = (6, 255);
pub const PASSWORD_LENGTH_RANGE: (usize, usize) = (8, 64);
pub static FIRSTNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Zа-яА-Я]*$").expect("Invalid firstname regex"));
pub static LASTNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Zа-яА-Я]*$").expect("Invalid lastname regex"));
pub static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._]*$").expect("Invalid username regex"));
// RFC2822
pub static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x22([^\x0d\x22\x5c\x80-\xff]|\x5c[\x00-\x7f])*\x22)(\x2e([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x22([^\x0d\x22\x5c\x80-\xff]|\x5c[\x00-\x7f])*\x22))*\x40([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x5b([^\x0d\x5b-\x5d\x80-\xff]|\x5c[\x00-\x7f])*\x5d)(\x2e([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x5b([^\x0d\x5b-\x5d\x80-\xff]|\x5c[\x00-\x7f])*\x5d))*$").expect("Invalid email regex"));



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub state: UserState,
    pub hashed_password: [u8],
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {

    pub fn new(
        username: String,
        email: String,
        state: UserState,
        hashed_password: &[u8],
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            first_name,
            last_name,
            state,
            hashed_password: *hashed_password.to_vec(),
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn update(
        &mut self,
        username: String,
        email: String,
        state: UserState,
        first_name: Option<String>,
        last_name: Option<String>,
        hashed_password: &[u8],
    ) {
        self.username = username;
        self.email = email;
        self.state = state;
        self.first_name = first_name;
        self.last_name = last_name;
        self.hashed_password = *hashed_password.to_vec();
        self.updated_at = Some(Utc::now());
    }
}
