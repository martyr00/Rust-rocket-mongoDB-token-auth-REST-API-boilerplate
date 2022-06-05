// TODO JWD TOKEN!!!
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_temp_jwt(uid: String) {
    let expiration = create_expiration(3600);

    let _claims = claims(uid, expiration);
}

pub fn create_jwt(uid: String) {
    let expiration = create_expiration(3600 * 7);

    let _claims = claims(uid, expiration);
}

fn claims(uid: String, exp: i64) -> Claims {
    Claims {
        sub: uid.to_owned(),
        exp: exp as usize,
    }
}

fn create_expiration(seconds: i64) -> i64 {
    Utc::now()
        .checked_add_signed(chrono::Duration::seconds(seconds))
        .expect("valid timestamp")
        .timestamp()
}
