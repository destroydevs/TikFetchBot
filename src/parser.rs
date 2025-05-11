use lazy_static::lazy_static;
use regex::Regex;

pub fn is_tiktok_url(message: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?ix)^(https?://)?\w{2,3}\.tiktok\.com/.+"
        ).unwrap();
    }
    RE.is_match(message)
}