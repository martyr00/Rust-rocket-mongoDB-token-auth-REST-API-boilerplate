use bcrypt::{hash, BcryptResult};
use rocket::http::Status;

pub fn get_valid_text(text: &str, max_size: usize, min_size: usize) -> bool {
    return if !text.is_empty() && text.len() <= max_size && text.len() >= min_size {
        true
    } else {
        false
    };
}

pub fn hash_text(text: &str, cost: u32) -> Result<String, Status> {
    return match hash(text, cost) {
        Ok(hash_text) => Ok(hash_text),
        Err(_) => Err(Status::BadRequest),
    };
}
