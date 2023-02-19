use crate::{
    db_constants::{KeyName, GSI1_NAME, GSI2_NAME, LSI1_NAME, TABLE_NAME},
    dto, DbClient,
};

use aws_sdk_dynamodb::model::AttributeValue;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Sort {
    Ascending,
    Descending,
}
impl Default for Sort {
    fn default() -> Self {
        Sort::Descending
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Count(String);
impl Default for Count {
    fn default() -> Self {
        Count("1".to_owned())
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UserDailyHighScoreQuery {
    user: String,
    game: String,
    stat: String,
    day: String,
    #[serde(default)]
    count: Count,
    #[serde(default)]
    sort: Sort,
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UserHighScoreQuery {
    user: String,
    game: String,
    stat: String,
    #[serde(default)]
    count: Count,
    #[serde(default)]
    sort: Sort,
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DailyHighScoreQuery {
    game: String,
    stat: String,
    day: String,
    #[serde(default)]
    count: Count,
    #[serde(default)]
    sort: Sort,
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UniversalHighScoreQuery {
    game: String,
    stat: String,
    #[serde(default)]
    count: Count,
    #[serde(default)]
    sort: Sort,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum StatQuery {
    UserDailyHighScoreQuery(UserDailyHighScoreQuery),
    UserHighScoreQuery(UserHighScoreQuery),
    DailyHighScoreQuery(DailyHighScoreQuery),
    UniversalHighScoreQuery(UniversalHighScoreQuery),
}

impl StatQuery {
    pub async fn execute(
        &self,
        db_client: DbClient,
    ) -> Result<Vec<dto::stat_view::StatView>, dto::query_error::QueryError> {
        match self {
            StatQuery::UserDailyHighScoreQuery(q) => {
                let partition_key = format!("User#{}", q.user);
                let sort_key_prefix = format!("Game#{}#Day#{}#Stat#{}", q.game, q.day, q.stat);

                query_db(
                    db_client,
                    None,
                    partition_key,
                    sort_key_prefix,
                    &q.count,
                    &q.sort,
                )
                .await
            }
            StatQuery::UserHighScoreQuery(q) => {
                let partition_key = format!("User#{}", q.user);
                let sort_key_prefix = format!("Game#{}#Stat#{}", q.game, q.stat);

                query_db(
                    db_client,
                    Some(LSI1_NAME),
                    partition_key,
                    sort_key_prefix,
                    &q.count,
                    &q.sort,
                )
                .await
            }
            StatQuery::DailyHighScoreQuery(q) => {
                let partition_key = format!("Game#{}#Stat#{}", q.game, q.stat);
                let sort_key_prefix = format!("Day#{}", q.day);

                query_db(
                    db_client,
                    Some(GSI1_NAME),
                    partition_key,
                    sort_key_prefix,
                    &q.count,
                    &q.sort,
                )
                .await
            }
            StatQuery::UniversalHighScoreQuery(q) => {
                let partition_key = format!("Game#{}#Stat#{}", q.game, q.stat);
                let sort_key_prefix = format!("Value#");

                query_db(
                    db_client,
                    Some(GSI2_NAME),
                    partition_key,
                    sort_key_prefix,
                    &q.count,
                    &q.sort,
                )
                .await
            }
        }
    }
}

async fn query_db<'a>(
    db_client: DbClient,
    index: Option<&str>,
    partition_key: String,
    sort_key_prefix: String,
    limit: &'a Count,
    sort: &'a Sort,
) -> Result<Vec<dto::stat_view::StatView<'a>>, dto::query_error::QueryError> {
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
        .scan_index_forward(matches!(sort, Sort::Ascending))
        .limit(limit.0.parse()?)
        .send()
        .await?;

    println!("{:?}", result);

    match result.items() {
        Some(items) => Ok(items.iter().map(dto::stat_view::StatView::from).collect()),
        None => Ok(vec![]),
    }
}
