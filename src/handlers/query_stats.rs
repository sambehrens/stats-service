use crate::{dto, DbClient};

pub async fn query_stats(
    query: dto::stat_query::StatQuery,
    db_client: DbClient,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = query.execute(db_client).await;

    match result {
        Ok(items) => Ok(warp::reply::with_status(
            warp::reply::json(&items),
            warp::http::StatusCode::OK,
        )),
        Err(e) => {
            println!("{:?}", e);
            Ok(warp::reply::with_status(
                warp::reply::json(&dto::error::Error::new("Failed to query")),
                warp::http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}
