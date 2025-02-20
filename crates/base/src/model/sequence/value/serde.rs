// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

fn serialize_single_field<V, S>(v: &V, serializer: S, struct_name: &'static str) -> Result<S::Ok, S::Error>
where
	S: Serializer,
	V: Serialize,
{
	let mut s = serializer.serialize_struct(struct_name, 1)?;
	s.serialize_field("value", v)?;
	s.end()
}

fn deserialize_single_field<'de, D, V>(deserializer: D) -> Result<V, D::Error>
where
	D: Deserializer<'de>,
	V: Deserialize<'de>,
{
	struct SingleFieldVisitor<V> {
		_marker: std::marker::PhantomData<V>,
	}

	impl<'de, V> Visitor<'de> for SingleFieldVisitor<V>
	where
		V: Deserialize<'de>,
	{
		type Value = V;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("a map with a 'value' field")
		}

		fn visit_map<M>(self, mut map: M) -> Result<V, M::Error>
		where
			M: MapAccess<'de>,
		{
			let mut value: Option<V> = None;

			while let Some(key) = map.next_key::<String>()? {
				if key == "value" {
					value = Some(map.next_value()?);
				} else {
					let _ = map.next_value::<de::IgnoredAny>()?;
				}
			}

			value.ok_or_else(|| de::Error::missing_field("value"))
		}
	}

	deserializer.deserialize_map(SingleFieldVisitor::<V> {
		_marker: std::marker::PhantomData,
	})
}

pub(crate) fn serialize_boolean<S>(v: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serialize_single_field(v, serializer, "Boolean")
}

pub(crate) fn deserialize_boolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: Deserializer<'de>,
{
	deserialize_single_field(deserializer)
}

pub(crate) fn serialize_count<S>(v: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serialize_single_field(v, serializer, "Count")
}

pub(crate) fn deserialize_count<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
	D: Deserializer<'de>,
{
	deserialize_single_field(deserializer)
}

pub(crate) fn serialize_percent<S>(v: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serialize_single_field(v, serializer, "Percent")
}

pub(crate) fn deserialize_percent<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
	D: Deserializer<'de>,
{
	deserialize_single_field(deserializer)
}

pub(crate) fn serialize_quote<S>(v: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serialize_single_field(v, serializer, "Quote")
}

pub(crate) fn deserialize_quote<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
	D: Deserializer<'de>,
{
	deserialize_single_field(deserializer)
}

pub(crate) fn serialize_sol<S>(v: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serialize_single_field(v, serializer, "Sol")
}

pub(crate) fn deserialize_sol<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
	D: Deserializer<'de>,
{
	deserialize_single_field(deserializer)
}

pub(crate) fn serialize_string<S>(v: &String, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serialize_single_field(v, serializer, "String")
}

pub(crate) fn deserialize_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
	D: Deserializer<'de>,
{
	deserialize_single_field(deserializer)
}

pub(crate) fn serialize_usd<S>(v: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serialize_single_field(v, serializer, "Usd")
}

pub(crate) fn deserialize_usd<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
	D: Deserializer<'de>,
{
	deserialize_single_field(deserializer)
}

#[cfg(test)]
mod tests {
	use crate::model::Value;

	#[test]
	fn boolean() {
		let serialised = serde_json::to_string(&Value::Boolean(true)).unwrap();
		assert_eq!(serialised, r#"{"type":"BOOLEAN","value":true}"#);
		assert_eq!(serde_json::from_str::<Value>(serialised.as_str()).unwrap(), Value::Boolean(true));
	}

	#[test]
	fn count() {
		let serialised = serde_json::to_string(&Value::Count(21)).unwrap();
		assert_eq!(serialised, r#"{"type":"COUNT","value":21}"#);
		assert_eq!(serde_json::from_str::<Value>(serialised.as_str()).unwrap(), Value::Count(21));
	}

	#[test]
	fn percent() {
		let serialised = serde_json::to_string(&Value::Percent(1.234)).unwrap();
		assert_eq!(serialised, r#"{"type":"PERCENT","value":1.234}"#);
		assert_eq!(serde_json::from_str::<Value>(serialised.as_str()).unwrap(), Value::Percent(1.234));
	}

	#[test]
	fn quote() {
		let serialised = serde_json::to_string(&Value::Quote(1.234)).unwrap();
		assert_eq!(serialised, r#"{"type":"QUOTE","value":1.234}"#);
		assert_eq!(serde_json::from_str::<Value>(serialised.as_str()).unwrap(), Value::Quote(1.234));
	}

	#[test]
	fn sol() {
		let serialised = serde_json::to_string(&Value::Sol(1.234)).unwrap();
		assert_eq!(serialised, r#"{"type":"SOL","value":1.234}"#);
		assert_eq!(serde_json::from_str::<Value>(serialised.as_str()).unwrap(), Value::Sol(1.234));
	}

	#[test]
	fn string() {
		let serialised = serde_json::to_string(&Value::String("nyanbot".to_string())).unwrap();
		assert_eq!(serialised, r#"{"type":"STRING","value":"nyanbot"}"#);
		assert_eq!(
			serde_json::from_str::<Value>(serialised.as_str()).unwrap(),
			Value::String("nyanbot".to_string())
		);
	}

	#[test]
	fn usd() {
		let serialised = serde_json::to_string(&Value::Usd(1.234)).unwrap();
		assert_eq!(serialised, r#"{"type":"USD","value":1.234}"#);
		assert_eq!(serde_json::from_str::<Value>(serialised.as_str()).unwrap(), Value::Usd(1.234));
	}
}
