use std::time::Duration;
use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use uuid::Uuid;
use crate::application::common::id_provider::IdProvider;
use crate::domain::models::user::UserState;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    pub user_id: Uuid,
    pub exp: usize
}


pub struct JwtTokenProcessor {
    private_key: String,
    public_key: String,
    header: Header,
    exp: TimeDelta,
}

impl JwtTokenProcessor {

    pub fn new(
        private_key: String,
        public_key: String,
        algorithm: Algorithm,
        exp: TimeDelta,
    ) -> Self {
        JwtTokenProcessor {
            private_key,
            public_key,
            header: Header::new(algorithm),
            exp,
        }
    }


    pub fn create_token(&self, user_id: Uuid) -> String {
        let exp = Utc::now() + Duration::from_secs(self.exp.num_seconds() as u64);

        let payload = JwtPayload {
            user_id,
            exp: exp.timestamp() as usize
        };

        match self.header.alg {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
                let token = encode(
                    &self.header,
                    &payload,
                    &EncodingKey::from_secret(self.private_key.as_ref())
                ).unwrap();
                return token;
            },
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
                let token = encode(
                    &self.header,
                    &payload,
                    &EncodingKey::from_rsa_pem(self.private_key.as_ref()).unwrap()
                ).unwrap();
                return token;
            },
            _ => panic!("Algorithm not supported")
        }
    }

    pub fn verify_token(&self, token: &str) -> Option<JwtPayload> {
        match self.header.alg {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
                match decode::<JwtPayload>(
                    token,
                    &DecodingKey::from_secret(self.public_key.as_ref()),
                    &Validation::new(self.header.alg)
                ) {
                    Ok(data) => Some(data.claims),
                    Err(_) => None
                }
            },
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
                match decode::<JwtPayload>(
                    token,
                    &DecodingKey::from_rsa_pem(self.public_key.as_ref()).unwrap(),
                    &Validation::new(self.header.alg)
                ) {
                    Ok(data) => Some(data.claims),
                    Err(_) => None
                }
            },
            _ => panic!("Algorithm not supported")
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub user_id: Uuid,
    pub user_state: UserState,
    pub permissions: Vec<String>,
    pub user_agent: String,
    pub ip: String,
}

struct TokenIdProvider {
    payload: Option<TokenPayload>,
    is_auth: bool
}

impl TokenIdProvider {

    pub fn new(
        jwt_access_token_processor: JwtTokenProcessor,
        jwt_refresh_token_processor: JwtTokenProcessor,
        access_token: String,
        refresh_token: String
    ) -> Self {

        let payload = match match jwt_access_token_processor.verify_token(&access_token) {
            Some(payload) => Some(payload),
            None => match jwt_refresh_token_processor.verify_token(&refresh_token) {
                Some(payload) => Some(payload),
                None => None
            }
        } {
            Some(payload) => Some(
                TokenPayload {
                    user_id: payload.user_id,
                    user_state: UserState::Active,
                    permissions: vec![],
                    user_agent: "".to_string(),
                    ip: "".to_string()
                }
            ),
            None => None
        };

        TokenIdProvider {
            is_auth: payload.is_some(),
            payload
        }
    }
}

impl IdProvider for TokenIdProvider {

    fn user_id(&self) -> Uuid {
        self.payload.as_ref().unwrap().user_id
    }

    fn user_state(&self) -> UserState {
        self.payload.as_ref().unwrap().user_state.clone()
    }

    fn permissions(&self) -> Vec<String> {
        self.payload.as_ref().unwrap().permissions.clone()
    }

    fn user_agent(&self) -> String {
        self.payload.as_ref().unwrap().user_agent.clone()
    }

    fn ip(&self) -> String {
        self.payload.as_ref().unwrap().ip.clone()
    }

    fn is_auth(&self) -> bool {
        self.is_auth
    }
}
