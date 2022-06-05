use crate::get_valid_text;
use crate::helper::get_valid_name;

pub mod authorization;
pub mod routes_models;

pub enum GetIsValidTwoStr {
    Ok,
    BadFirst,
    BadSecond,
}

pub fn valid_two_str(
    first_str: &str,
    second_str: &str,
    max_first: usize,
    min_first: usize,
    max_second: usize,
    min_second: usize,
) -> GetIsValidTwoStr {
    if get_valid_text(first_str, max_first, min_first) {
        if get_valid_text(second_str, max_second, min_second) {
            GetIsValidTwoStr::Ok
        } else {
            GetIsValidTwoStr::BadSecond
        }
    } else {
        GetIsValidTwoStr::BadFirst
    }
}

pub fn valid_two_names(
    first_str: &str,
    second_str: &str,
    max_first: usize,
    min_first: usize,
    max_second: usize,
    min_second: usize,
) -> GetIsValidTwoStr {
    if get_valid_name(first_str, max_first, min_first) {
        if get_valid_name(second_str, max_second, min_second) {
            GetIsValidTwoStr::Ok
        } else {
            GetIsValidTwoStr::BadSecond
        }
    } else {
        GetIsValidTwoStr::BadFirst
    }
}
