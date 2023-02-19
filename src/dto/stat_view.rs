use std::{borrow::Cow, collections::HashMap, str::FromStr};

use aws_sdk_dynamodb::model::AttributeValue;

use crate::time_utils;

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatView<'a> {
    user: Cow<'a, str>,
    game: Cow<'a, str>,
    stat: Cow<'a, str>,
    value: f64,
    added_timestamp: u128,
    day: u128,
}

fn convert_value_to_storage_value(value: f64) -> String {
    match value.is_sign_positive() {
        true => format!("{}#{}", to_bin_pos(value), value),
        false => format!("{}#{}", to_bin_neg(value), value),
    }
}

fn to_bin_pos(n: f64) -> String {
    format!("{:16x}", n.to_bits() ^ (1 << 63_u64))
}

fn to_bin_neg(n: f64) -> String {
    format!("{:016x}", n.to_bits() ^ u64::MAX)
}

impl<'a> StatView<'a> {
    pub fn new(user: &'a str, game: &'a str, stat: &'a str, value: f64, day: Option<u128>) -> Self {
        Self {
            user: Cow::Borrowed(&user),
            game: Cow::Borrowed(&game),
            stat: Cow::Borrowed(&stat),
            value,
            day: day.unwrap_or(time_utils::get_current_day()),
            added_timestamp: time_utils::get_current_timestamp(),
        }
    }

    pub fn as_db_item(&self) -> HashMap<String, AttributeValue> {
        let storage_value = convert_value_to_storage_value(self.value);

        HashMap::from([
            (
                "PK".to_string(),
                AttributeValue::S(format!("User#{}", self.user)),
            ),
            (
                "SK".to_string(),
                AttributeValue::S(format!(
                    "Game#{}#Day#{}#Stat#{}#Value#{}#Timestamp#{}",
                    self.game, self.day, self.stat, storage_value, self.added_timestamp
                )),
            ),
            (
                "LSI-SK".to_string(),
                AttributeValue::S(format!(
                    "Game#{}#Stat#{}#Value#{}",
                    self.game, self.stat, self.value
                )),
            ),
            (
                "GSI1-PK".to_string(),
                AttributeValue::S(format!("Game#{}#Stat#{}", self.game, self.stat)),
            ),
            (
                "GSI1-SK".to_string(),
                AttributeValue::S(format!(
                    "Day#{}#Value#{}#Timestamp#{}",
                    self.day, storage_value, self.added_timestamp
                )),
            ),
            (
                "GSI2-PK".to_string(),
                AttributeValue::S(format!("Game#{}#Stat#{}", self.game, self.stat)),
            ),
            (
                "GSI2-SK".to_string(),
                AttributeValue::S(format!(
                    "Value#{}#Timestamp#{}",
                    storage_value, self.added_timestamp
                )),
            ),
            ("User".to_string(), AttributeValue::S(self.user.to_string())),
            ("Game".to_string(), AttributeValue::S(self.game.to_string())),
            ("Stat".to_string(), AttributeValue::S(self.stat.to_string())),
            (
                "Value".to_string(),
                AttributeValue::N(self.value.to_string()),
            ),
            ("Day".to_string(), AttributeValue::N(self.day.to_string())),
            (
                "Timestamp".to_string(),
                AttributeValue::N(self.added_timestamp.to_string()),
            ),
        ])
    }
}

impl<'a> From<&HashMap<String, AttributeValue>> for StatView<'a> {
    fn from(value: &HashMap<String, AttributeValue>) -> Self {
        fn get_string<'a>(value: &HashMap<String, AttributeValue>, key: &str) -> Cow<'a, str> {
            Cow::Owned(
                value
                    .get(key)
                    .expect(&format!("{} not in db item", key))
                    .as_s()
                    .expect(&format!(
                        "Cannot convert {} db field to valid Stat {} field",
                        key, key
                    ))
                    .to_owned(),
            )
        }

        fn get_u128(value: &HashMap<String, AttributeValue>, key: &str) -> u128 {
            value
                .get(key)
                .expect(&format!("{} not in db item", key))
                .as_n()
                .expect(&format!(
                    "Cannot convert {} db field to valid Stat {} field",
                    key, key
                ))
                .parse()
                .expect(&format!(
                    "Cannot convert {} db field to valid Stat {} field",
                    key, key
                ))
        }

        fn get_f64(value: &HashMap<String, AttributeValue>, key: &str) -> f64 {
            value
                .get(key)
                .expect(&format!("{} not in db item", key))
                .as_n()
                .expect(&format!(
                    "Cannot convert {} db field to valid Stat {} field",
                    key, key
                ))
                .parse()
                .expect(&format!(
                    "Cannot convert {} db field to valid Stat {} field",
                    key, key
                ))
        }

        Self {
            user: get_string(value, "User"),
            game: get_string(value, "Game"),
            stat: get_string(value, "Stat"),
            value: get_f64(value, "Value"),
            added_timestamp: get_u128(value, "Timestamp"),
            day: get_u128(value, "Day"),
        }
    }
}
