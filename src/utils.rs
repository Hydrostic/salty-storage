use lazy_static::lazy_static;

use regex::Regex;

fn validate_email(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();
    }
    RE.is_match(input)
}
fn validate_phone(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(13[0-9]|14[5|7]|15[0|1|2|3|5|6|7|8|9]|18[0|1|2|3|5|6|7|8|9])\d{8}$")
                .unwrap();
    }
    RE.is_match(input)
}
