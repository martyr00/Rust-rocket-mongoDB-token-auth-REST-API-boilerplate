pub mod delete_user;
pub mod hello_name;
pub mod patch_user;
pub mod refresh_tokens;
pub mod get_data_user;

enum HelloNameError {
    OnlyLogin(String),
    NoOnlyLogin(String),
    ErrorID,
}
