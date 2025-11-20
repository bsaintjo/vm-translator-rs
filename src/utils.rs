use std::borrow::Cow;

pub type StringLike = Cow<'static, str>;

pub fn remove_whitespace_comments(mut s: &str) -> Option<&str> {
    if let Some(comment_idx) = s.find("//") {
        s = &s[..comment_idx];
    }

    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
