use rustica::datatypes::either::Either;
use rustica::pvec::PersistentVector;
use rustica::traits::functor::Functor;

use crate::data_processing::json_filter::User;

/// JSON filter using Either for monadic error handling and PVec for filtering.
/// Demonstrates: Either::from_result → fmap_owned chain → PVec::filter → unwrap.
pub fn filter_users_from_str_rustica(data: &str) -> Vec<User> {
    let parsed: Either<String, Vec<User>> =
        Either::from_result(serde_json::from_str::<Vec<User>>(data).map_err(|e| e.to_string()));

    let filtered = parsed.fmap_owned(|users| {
        let pv = PersistentVector::from_slice(&users);
        pv.filter(|u| u.active && u.age >= 30).to_vec()
    });

    filtered.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_processing::json_filter;

    fn sample_json() -> &'static str {
        r#"[
            {"id":1,"name":"Alice","age":35,"city":"Tokyo","active":true},
            {"id":2,"name":"Bob","age":25,"city":"Osaka","active":true},
            {"id":3,"name":"Carol","age":40,"city":"Kyoto","active":false}
        ]"#
    }

    #[test]
    fn should_filter_active_users_over_30() {
        let result = filter_users_from_str_rustica(sample_json());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Alice");
    }

    #[test]
    fn should_match_original_filter() {
        let original = json_filter::filter_users_from_str(sample_json());
        let rustica = filter_users_from_str_rustica(sample_json());
        assert_eq!(original.len(), rustica.len());
        assert_eq!(original[0].id, rustica[0].id);
    }

    #[test]
    fn should_handle_empty_array() {
        let result = filter_users_from_str_rustica("[]");
        assert!(result.is_empty());
    }
}
