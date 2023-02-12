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

pub async fn query_stats(query: StatQuery) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("{:?}", query))
}
