pub fn to_title(s: &impl ToString) -> String {
    let mut s = s.to_string();
    s[..1].make_ascii_uppercase();
    s
}

pub fn to_time(seconds: i64) -> String {
    let duration = chrono::Duration::seconds(seconds);
    format!(
        "{:02}:{:02}",
        duration.num_minutes() % 99,
        duration.num_seconds() % 60
    )
}
