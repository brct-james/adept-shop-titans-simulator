use serde::de::{self, Unexpected, Visitor};
use serde::Deserializer;

/// Converts a string to a boolean based on truthy and falsy values.
///
/// Designed to be used as #[serde(deserialize_with = "bool_from_str")]
pub fn bool_from_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct BoolVisitor;
    impl Visitor<'_> for BoolVisitor {
        type Value = bool;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                formatter,
                "truthy (t, true, 1, on, y, yes) or falsey (f, false, 0, off, n, no) string"
            )
        }
        fn visit_str<E: de::Error>(self, s: &str) -> Result<bool, E> {
            match s {
                "t" | "T" | "true" | "True" | "1" | "on" | "On" | "y" | "Y" | "yes" | "Yes" => {
                    Ok(true)
                }
                "f" | "F" | "false" | "False" | "0" | "off" | "Off" | "n" | "N" | "no" | "No" => {
                    Ok(false)
                }
                other => {
                    // handle weird mixed-case spellings like tRue or nO
                    match other.to_lowercase().as_str() {
                        "true" | "on" | "yes" => Ok(true),
                        "false" | "off" | "no" => Ok(false),
                        other => Err(de::Error::invalid_value(Unexpected::Str(other), &self)),
                    }
                }
            }
        }
    }

    deserializer.deserialize_str(BoolVisitor)
}
