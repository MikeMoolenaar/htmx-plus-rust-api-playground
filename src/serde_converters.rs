use chrono::NaiveDate;
use html_escape;
use serde::{Deserialize, Deserializer};

pub fn date_to_timestamp<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)?;
    let ts = dt.and_hms_opt(0, 0, 0).unwrap().timestamp();
    return Ok(ts);
}

pub fn html_encode<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let s = html_escape::encode_text(&s);
    return Ok(s.to_string());
}
