use chrono::Utc;

pub fn create_temp_jwt() {
    let expiration = create_expiration(3600);
}

pub fn create_jwt() {
    let expiration = create_expiration(3600 * 7);
}

fn create_expiration(seconds: i64) -> i64 {
    Utc::now()
        .checked_add_signed(chrono::Duration::seconds(seconds))
        .expect("valid timestamp")
        .timestamp()
}
