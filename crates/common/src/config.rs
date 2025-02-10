// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original MIT License Copyright (c) blockworks-foundation 2024.

use std::env;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Deserializer};
use toml::Value;

/// Error types that can occur during the resolution of a `ConfigValue`.
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    /// The configuration value was not found.
    NotFound,
    /// The environment variable was not found.
    EnvNotFound(String),
    /// The file was not found.
    FileNotFound(String),
    /// The file could not be read.
    UnableToReadFile(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::EnvNotFound(name) => write!(f, "Env not found: {}", name),
            ConfigError::FileNotFound(path) => write!(f, "File not found: {}", path),
            ConfigError::NotFound => write!(f, "Not found"),
            ConfigError::UnableToReadFile(path) => write!(f, "Unable to read file: {}", path),
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Clone, Default, Debug, PartialEq)]
/// Represents a configuration value that can come from various sources.
pub enum ConfigValue {
    /// No configuration value provided.
    #[default]
    None,
    /// A direct value provided as a string.
    Value(String),
    /// A configuration value which will read from an environment variable
    Env(String),
    /// A configuration which will read from file
    File(String),
}

/// Resolves the `ConfigValue`, returning the contained value or a provided default if resolution fails.
///
/// # Arguments
///
/// * `default` - A default value to return if the configuration value cannot be resolved.
pub trait ResolveOr<T> {
    fn resolve_or(&self, default: T) -> T;
}

impl ConfigValue {
    /// Creates a `ConfigValue` based on the input string.
    ///
    /// - If the string is empty, it returns `ConfigValue::None`.
    /// - If the string starts with a `$`, it returns `ConfigValue::Env` with the rest of the string
    ///   as the environment variable name.
    /// - If the string is an absolute file path, it returns `ConfigValue::File`.
    /// - Otherwise, it returns `ConfigValue::Value` with the provided string.
    pub fn of<T: AsRef<str>>(str: T) -> Self {
        let str = str.as_ref();
        if str.is_empty() {
            return ConfigValue::None;
        }

        if Self::is_env(str) {
            return ConfigValue::Env(str[1..].to_string());
        }

        if Self::is_file(str) {
            return ConfigValue::File(str.to_string());
        }

        Self::Value(str.into())
    }

    /// Resolves the `ConfigValue`, returning the contained value or panicking if resolution fails.
    ///
    /// This method attempts to resolve the value of the `ConfigValue`. If the value is `None` or
    /// resolution fails (e.g., environment variable not found or file read error), it will panic.
    ///
    /// # Panics
    ///
    /// Panics if the configuration value cannot be resolved.
    pub fn resolve(&self) -> String {
        self.try_resolve().expect("Failed to resolve value")
    }

    /// Tries to resolve the `ConfigValue`, returning a `Result`.
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError` if the value cannot be resolved (e.g., environment variable not found,
    /// file not found, or unable to read file).
    pub fn try_resolve(&self) -> Result<String, ConfigError> {
        match self {
            Self::None => Err(ConfigError::NotFound),
            Self::Value(value) => Ok(value.clone()),
            Self::Env(name) => env::var(name).map_err(|_| ConfigError::EnvNotFound(name.clone())),
            Self::File(path) => {
                let mut file = File::open(path).map_err(|_| ConfigError::FileNotFound(path.clone()))?;
                let mut contents = String::new();
                file.read_to_string(&mut contents).map_err(|_| ConfigError::UnableToReadFile(path.clone()))?;
                Ok(contents)
            }
        }
    }

    fn is_env(str: &str) -> bool {
        str.chars().next().is_some_and(|c| c == '$')
    }

    fn is_file(str: &str) -> bool {
        Path::new(str).is_absolute()
    }
}

impl ResolveOr<bool> for ConfigValue {
    /// Resolves the `ConfigValue`, returning the contained value or a provided default if resolution fails.
    /// It never fails and will fall back to the default value
    /// # Arguments
    ///
    /// * `default` - A default value to return if the configuration value cannot be resolved.
    fn resolve_or(&self, default: bool) -> bool {
        self.try_resolve().map(|x| x.parse::<bool>().unwrap_or(default)).unwrap_or(default)
    }
}

impl ResolveOr<String> for ConfigValue {
    /// Resolves the `ConfigValue`, returning the contained value or a provided default if resolution fails.
    /// It never fails and will fall back to the default value
    /// # Arguments
    ///
    /// * `default` - A default value to return if the configuration value cannot be resolved.
    fn resolve_or(&self, default: String) -> String {
        self.try_resolve().unwrap_or(default)
    }
}

impl ResolveOr<usize> for ConfigValue {
    /// Resolves the `ConfigValue`, returning the contained value or a provided default if resolution fails.
    /// It never fails and will fall back to the default value
    /// # Arguments
    ///
    /// * `default` - A default value to return if the configuration value cannot be resolved.
    fn resolve_or(&self, default: usize) -> usize {
        self.try_resolve().map(|x| x.parse::<usize>().unwrap_or(default)).unwrap_or(default)
    }
}

impl<'de> Deserialize<'de> for ConfigValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match toml::value::Value::deserialize(deserializer) {
            Ok(value) => match value {
                Value::String(string_value) => ConfigValue::of(string_value),
                other => {
                    return Err(serde::de::Error::custom(format!("Config reader: expected string, found {:?}", other)));
                }
            },
            Err(_) => ConfigValue::None,
        })
    }
}

#[cfg(test)]
mod tests {
	use lazy_static::lazy_static;
	use std::env;
	use std::io::Write;
	use std::path::PathBuf;
	use std::sync::atomic::{AtomicU8, Ordering};

	use crate::config::ConfigError::EnvNotFound;
	use crate::config::{ConfigError, ConfigValue};

	#[test]
    fn none() {
        let result = ConfigValue::of("");
        assert_eq!(result, ConfigValue::None)
    }

    #[test]
    fn none_try_resolve() {
        let result = ConfigValue::None.try_resolve();
        assert_eq!(result, Err(ConfigError::NotFound));
    }

    #[test]
    #[should_panic]
    fn none_resolve_panics() {
        ConfigValue::None.resolve();
    }

    #[test]
    fn value() {
        let test_instance = ConfigValue::of("sweet_bot_var");

        let result = test_instance.resolve();
        assert_eq!(result, "sweet_bot_var");

        let result = test_instance.try_resolve();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "sweet_bot_var")
    }

    #[test]
    fn env() {
        unsafe {
            std::env::set_var("SWEET_BOT", "yes");
            let test_instance = ConfigValue::of("$SWEET_BOT");

            let result = test_instance.resolve();
            assert_eq!(result, "yes");

            let result = test_instance.try_resolve();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "yes");

            std::env::remove_var("SWEET_BOT");
            let result = test_instance.try_resolve();
            assert_eq!(result.err().unwrap(), EnvNotFound("SWEET_BOT".to_string()));
        }
    }

    #[test]
    #[should_panic]
    fn env_resolve_panics() {
        ConfigValue::Env(String::from("VARIABLE")).resolve();
    }

    #[test]
    fn file() {
        let temp_file_path = create_temp_file();
        let mut temp_file = std::fs::File::create(&temp_file_path).unwrap();
        temp_file.write_all("SWEETEST_BOT".as_ref()).unwrap();

        let test_instance = ConfigValue::of(std::fs::canonicalize(&temp_file_path).unwrap().as_os_str().to_str().unwrap());

        let result = test_instance.resolve();
        assert_eq!(result, "SWEETEST_BOT");

        let result = test_instance.try_resolve();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "SWEETEST_BOT");

        std::fs::remove_file(&temp_file_path).unwrap();

        let result = test_instance.try_resolve();
        assert!(result.is_err());

        match result.err().unwrap() {
            ConfigError::FileNotFound(_) => {}
            _ => panic!("Expected to be unable to find file"),
        }
    }

    #[test]
    #[should_panic]
    fn file_resolve_panics() {
        ConfigValue::File(String::from("/does/not/exists")).resolve();
    }

    mod is_env {
		use crate::config::ConfigValue;

		#[test]
        fn ok() {
            for e in ["$ENV", "$nice_nicer"] {
                assert!(ConfigValue::is_env(e), "expect {} to be an env", e)
            }
        }

        #[test]
        fn not_env() {
            for e in ["x$d", "NOT_ENV", "", " "] {
                assert!(!ConfigValue::is_env(e), "expect {} not to be an env", e)
            }
        }
    }

    mod is_file {
		use crate::config::ConfigValue;

		#[test]
        fn ok() {
            for f in ["/", "/some/absolute/path.txt"] {
                assert!(ConfigValue::is_file(f), "expect {} to be a file", f)
            }
        }

        #[test]
        fn not_file() {
            for f in [
                // "x$d",
                "NOT_A_FILE",
                "",
                " ",
                "*:?<>|",
                "relative/path.txt",
            ] {
                assert!(!ConfigValue::is_file(f), "expect {} not to be a file", f)
            }
        }
    }

    lazy_static! {
        static ref COUNTER: AtomicU8 = AtomicU8::new(0);
    }

    fn create_temp_file() -> PathBuf {
        let mut temp_dir = env::temp_dir();
        temp_dir.push(format!("test_file_{}.txt", COUNTER.fetch_add(1, Ordering::Relaxed)));
        temp_dir
    }

    mod resolve_or {
        mod string {
			use std::io::Write;

			use crate::config::tests::create_temp_file;
			use crate::{ConfigValue, ResolveOr};

			#[test]
            fn none() {
                assert_eq!("default", ConfigValue::None.resolve_or("default".to_string()))
            }

            #[test]
            fn value() {
                assert_eq!("value", ConfigValue::Value("value".to_string()).resolve_or("default".to_string()))
            }

            #[test]
            fn env() {
                unsafe {
                    std::env::set_var("SWEET_BOT_VAR", "yes");
                    let test_instance = ConfigValue::of("$SWEET_BOT_VAR");
                    assert_eq!("yes", test_instance.resolve_or("default".to_string()));

                    std::env::remove_var("SWEET_BOT_VAR");
                    assert_eq!("default", test_instance.resolve_or("default".to_string()));
                }
            }

            #[test]
            fn file() {
                let temp_file_path = create_temp_file();
                let mut temp_file = std::fs::File::create(&temp_file_path).unwrap();
                temp_file.write_all("SWEETEST_BOT".as_ref()).unwrap();

                let test_instance = ConfigValue::of(std::fs::canonicalize(&temp_file_path).unwrap().as_os_str().to_str().unwrap());
                assert_eq!("SWEETEST_BOT", test_instance.resolve_or("default".to_string()));

                std::fs::remove_file(&temp_file_path).unwrap();
                assert_eq!("default", test_instance.resolve_or("default".to_string()));
            }
        }

        mod bool {
			use crate::{ConfigValue, ResolveOr};

			#[test]
            fn none() {
                assert!(ConfigValue::None.resolve_or(true))
            }

            #[test]
            fn value() {
                assert!(!ConfigValue::Value(false.to_string()).resolve_or(true))
            }
        }

        mod usize {
			use crate::{ConfigValue, ResolveOr};

			#[test]
            fn none() {
                assert_eq!(1337, ConfigValue::None.resolve_or(1337))
            }

            #[test]
            fn value() {
                assert_eq!(2442, ConfigValue::Value("2442".to_string()).resolve_or(2442))
            }
        }
    }

    mod serde {
		use serde::Deserialize;

		use crate::config::ConfigValue;

		#[derive(Deserialize)]
        struct TestStruct {
            value: ConfigValue,
        }

        #[test]
        fn missing_value() {
            let result: TestStruct = toml::from_str(r#""#).unwrap();
            assert_eq!(result.value, ConfigValue::None)
        }

        #[test]
        fn empty_string() {
            let result: TestStruct = toml::from_str(r#"value=''"#).unwrap();
            assert_eq!(result.value, ConfigValue::None)
        }

        #[test]
        fn env() {
            let result: TestStruct = toml::from_str(r#"value='$VALUE'"#).unwrap();
            assert_eq!(result.value, ConfigValue::Env("VALUE".to_string()))
        }

        #[test]
        fn file() {
            let result: TestStruct = toml::from_str(r#"value='/test/file.txt'"#).unwrap();
            assert_eq!(result.value, ConfigValue::File("/test/file.txt".to_string()))
        }
    }
}
