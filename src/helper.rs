use crate::database::connect_to_db::MongoDB;
use crate::database::FindUser;
use bcrypt::hash;
use rocket::http::Status;

pub fn get_valid_text(text: &str, max_size: usize, min_size: usize) -> bool {
    return if !text.is_empty() && text.len() <= max_size && text.len() >= min_size {
        true
    } else {
        false
    };
}

pub fn get_valid_name(text: &str, max_size: usize, min_size: usize) -> bool {
    return if text.is_empty() || text.len() <= max_size && text.len() >= min_size {
        true
    } else {
        false
    };
}

pub fn hash_text(text: String, cost: u32) -> Result<String, Status> {
    return match hash(text, cost) {
        Ok(hash_text) => Ok(hash_text),
        Err(_) => Err(Status::BadRequest),
    };
}

pub async fn find_user_by_login_and_mail(database: &MongoDB, mail: &str, login: &str) -> FindUser {
    match database
        .find_user_by("login".to_string(), login.to_string())
        .await
    {
        Ok(None) => match database
            .find_user_by("mail".to_string(), mail.to_string())
            .await
        {
            Ok(None) => FindUser::UserNotFound,
            Ok(Some(_)) => FindUser::UserFoundByEmail,
            Err(_) => FindUser::UserFoundByEmail,
        },
        Ok(Some(_)) => FindUser::UserFoundByLogin,
        Err(_) => FindUser::UserFoundByLogin,
    }
}
