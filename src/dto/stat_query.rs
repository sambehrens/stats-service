use crate::{
    db_constants::{KeyName, GSI1_NAME, GSI2_NAME, LSI_NAME, TABLE_NAME},
    dto, DbClient,
};

use aws_sdk_dynamodb::model::AttributeValue;

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, untagged)]
pub enum StatQuery {
    #[serde(rename_all = "camelCase")]
    UserDailyHighScoreQuery {
        user: String,
        game: String,
        stat: String,
        day: String,
        count: String,
    },

    #[serde(rename_all = "camelCase")]
    UserHighScoreQuery {
        user: String,
        game: String,
        stat: String,
        count: String,
    },

    #[serde(rename_all = "camelCase")]
    DailyHighScoreQuery {
        game: String,
        stat: String,
        day: String,
        count: String,
    },

    #[serde(rename_all = "camelCase")]
    UniversalHighScoreQuery {
        game: String,
        stat: String,
        count: String,
    },
}

impl StatQuery {
    pub async fn execute(
        &self,
        db_client: DbClient,
    ) -> Result<Vec<dto::stat_view::StatView>, dto::query_error::QueryError> {
        match self {
            StatQuery::UserDailyHighScoreQuery {
                user,
                game,
                stat,
                day,
                count,
            } => {
                let partition_key = format!("User#{}", user);
                let sort_key_prefix = format!("Game#{}#Day#{}#Stat#{}", game, day, stat);

                query_db(
                    db_client,
                    None,
                    partition_key,
                    sort_key_prefix,
                    count.parse()?,
                )
                .await
            }
            StatQuery::UserHighScoreQuery {
                user,
                game,
                stat,
                count,
            } => {
                let partition_key = format!("User#{}", user);
                let sort_key_prefix = format!("Game#{}#Stat#{}", game, stat);

                query_db(
                    db_client,
                    Some(LSI_NAME),
                    partition_key,
                    sort_key_prefix,
                    count.parse()?,
                )
                .await
            }
            StatQuery::DailyHighScoreQuery {
                game,
                stat,
                day,
                count,
            } => {
                let partition_key = format!("Game#{}#Stat#{}", game, stat);
                let sort_key_prefix = format!("Day#{}", day);

                query_db(
                    db_client,
                    Some(GSI1_NAME),
                    partition_key,
                    sort_key_prefix,
                    count.parse()?,
                )
                .await
            }
            StatQuery::UniversalHighScoreQuery { game, stat, count } => {
                let partition_key = format!("Game#{}#Stat#{}", game, stat);
                let sort_key_prefix = format!("Value#");

                query_db(
                    db_client,
                    Some(GSI2_NAME),
                    partition_key,
                    sort_key_prefix,
                    count.parse()?,
                )
                .await
            }
        }
    }
}

async fn query_db(
    db_client: DbClient,
    index: Option<&str>,
    partition_key: String,
    sort_key_prefix: String,
    limit: i32,
) -> Result<Vec<dto::stat_view::StatView>, dto::query_error::QueryError> {
    let key_names = KeyName::from_index_name(index);
    let (pk_name, sk_name) = key_names.as_tuple();

    let result = db_client
        .query()
        .table_name(TABLE_NAME)
        .set_index_name(index.map(str::to_string))
        .key_condition_expression("#pk = :pk AND begins_with (#sk, :sk_prefix)")
        .expression_attribute_names("#pk", pk_name)
        .expression_attribute_names("#sk", sk_name)
        .expression_attribute_values(":pk", AttributeValue::S(partition_key))
        .expression_attribute_values(":sk_prefix", AttributeValue::S(sort_key_prefix))
        .scan_index_forward(false)
        .limit(limit)
        .send()
        .await?;

    println!("{:?}", result);

    match result.items() {
        Some(items) => Ok(items.iter().map(dto::stat_view::StatView::from).collect()),
        None => Ok(vec![]),
    }
}
