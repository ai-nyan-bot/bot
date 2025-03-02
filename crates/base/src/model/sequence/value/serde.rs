// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#[cfg(test)]
mod tests {
    use crate::model::Value;
    use common::model::TimeUnit;

    #[test]
    fn boolean() {
        let serialised = serde_json::to_string(&Value::boolean(true)).unwrap();
        assert_eq!(serialised, r#"{"type":"BOOLEAN","value":true}"#);
        assert_eq!(
            serde_json::from_str::<Value>(serialised.as_str()).unwrap(),
            Value::boolean(true)
        );
    }

    #[test]
    fn count() {
        let serialised = serde_json::to_string(&Value::count(21)).unwrap();
        assert_eq!(serialised, r#"{"type":"COUNT","value":21}"#);
        assert_eq!(
            serde_json::from_str::<Value>(serialised.as_str()).unwrap(),
            Value::count(21)
        );
    }

    #[test]
    fn duration() {
        let serialised = serde_json::to_string(&Value::duration(21, TimeUnit::Minute)).unwrap();
        assert_eq!(
            serialised,
            r#"{"type":"DURATION","value":21,"unit":"MINUTE"}"#
        );
        assert_eq!(
            serde_json::from_str::<Value>(serialised.as_str()).unwrap(),
            Value::duration(21, TimeUnit::Minute)
        );
    }

    #[test]
    fn percent() {
        let serialised = serde_json::to_string(&Value::percent(1.234)).unwrap();
        assert_eq!(serialised, r#"{"type":"PERCENT","value":1.234}"#);
        assert_eq!(
            serde_json::from_str::<Value>(serialised.as_str()).unwrap(),
            Value::percent(1.234)
        );
    }

    #[test]
    fn quote() {
        let serialised = serde_json::to_string(&Value::quote(1.234)).unwrap();
        assert_eq!(serialised, r#"{"type":"QUOTE","value":1.234}"#);
        assert_eq!(
            serde_json::from_str::<Value>(serialised.as_str()).unwrap(),
            Value::quote(1.234)
        );
    }

    #[test]
    fn sol() {
        let serialised = serde_json::to_string(&Value::sol(1.234)).unwrap();
        assert_eq!(serialised, r#"{"type":"SOL","value":1.234}"#);
        assert_eq!(
            serde_json::from_str::<Value>(serialised.as_str()).unwrap(),
            Value::sol(1.234)
        );
    }

    #[test]
    fn string() {
        let serialised = serde_json::to_string(&Value::string("nyanbot".to_string())).unwrap();
        assert_eq!(serialised, r#"{"type":"STRING","value":"nyanbot"}"#);
        assert_eq!(
            serde_json::from_str::<Value>(serialised.as_str()).unwrap(),
            Value::string("nyanbot".to_string())
        );
    }

    #[test]
    fn usd() {
        let serialised = serde_json::to_string(&Value::usd(1.234)).unwrap();
        assert_eq!(serialised, r#"{"type":"USD","value":1.234}"#);
        assert_eq!(
            serde_json::from_str::<Value>(serialised.as_str()).unwrap(),
            Value::usd(1.234)
        );
    }
}
