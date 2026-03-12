use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub age: u32,
    pub city: String,
    pub active: bool,
}

/// Parse JSON array of users and filter to active users aged >= 30.
pub fn filter_users_from_file(path: &Path) -> Vec<User> {
    let data = fs::read_to_string(path).expect("failed to read file");
    filter_users_from_str(&data)
}

/// Parse and filter from a string (for criterion benchmarks).
pub fn filter_users_from_str(data: &str) -> Vec<User> {
    let users: Vec<User> = serde_json::from_str(data).expect("failed to parse JSON");
    users
        .into_iter()
        .filter(|u| u.active && u.age >= 30)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_filter_active_users_over_30() {
        let json = r#"[
            {"id":1,"name":"Alice","age":35,"city":"Tokyo","active":true},
            {"id":2,"name":"Bob","age":25,"city":"Osaka","active":true},
            {"id":3,"name":"Carol","age":40,"city":"Kyoto","active":false}
        ]"#;
        let result = filter_users_from_str(json);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Alice");
    }
}
