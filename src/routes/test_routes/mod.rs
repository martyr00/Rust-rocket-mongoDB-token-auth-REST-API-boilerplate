pub mod hello_name;

enum HelloNameError {
    OnlyLogin(String),
    NoOnlyLogin(String),
    ErrorID,
    ErrorUnknown,
}
