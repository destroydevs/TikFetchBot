use lazy_static::lazy_static;
use regex::Regex;

pub fn is_tiktok_url(message: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?ix)^https?://
            (?:www\.|m\.|vm\.)?      # Subdomain
            tiktok\.com/
            (?:                       # Path options:
                @[\w-]+/video/\d+     # Standard format with video
                | t/\d+               # Format with /t/
                | \w+                 # Short ID for VM
            )
            /?$                       # Optional finishing slash"
        ).unwrap();
    }
    RE.is_match(message)
}