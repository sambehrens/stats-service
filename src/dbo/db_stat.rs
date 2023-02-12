use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;

pub struct DbStat {
    pk: AttributeValue,
    sk: AttributeValue,
    lsi_sk: AttributeValue,
    gsi1_pk: AttributeValue,
    gsi1_sk: AttributeValue,
    gsi2_pk: AttributeValue,
    gsi2_sk: AttributeValue,
    user: AttributeValue,
    game: AttributeValue,
    stat: AttributeValue,
    value: AttributeValue,
    day: AttributeValue,
    added_timestamp: AttributeValue,
}

impl DbStat {
    pub fn new(
        user: &str,
        game: &str,
        stat: &str,
        value: f64,
        day: u128,
        added_timestamp: u128,
    ) -> Self {
        Self {
            pk: AttributeValue::S(format!("User#{}", user)),
            sk: AttributeValue::S(format!(
                "Game#{}#Day#{}#Stat#{}#Value#{}#Timestamp#{}",
                game, day, stat, value, added_timestamp
            )),
            lsi_sk: AttributeValue::S(format!("Game#{}#Stat#{}#Value#{}", game, stat, value)),
            gsi1_pk: AttributeValue::S(format!("Game#{}#Stat#{}", game, stat)),
            gsi1_sk: AttributeValue::S(format!(
                "Day#{}#Value#{}#Timestamp#{}",
                day, value, added_timestamp
            )),
            gsi2_pk: AttributeValue::S(format!("Game#{}#Stat#{}", game, stat)),
            gsi2_sk: AttributeValue::S(format!("Value#{}#Timestamp#{}", value, added_timestamp)),
            user: AttributeValue::S(user.to_string()),
            game: AttributeValue::S(game.to_string()),
            stat: AttributeValue::S(stat.to_string()),
            value: AttributeValue::N(value.to_string()),
            day: AttributeValue::N(day.to_string()),
            added_timestamp: AttributeValue::N(added_timestamp.to_string()),
        }
    }

}
