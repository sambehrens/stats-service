use crate::{db_constants, dto, DbClient};

pub async fn add_stat(
    body: dto::add_stat_request::AddStatRequest,
    client: DbClient,
) -> Result<impl warp::Reply, warp::Rejection> {
    let stat = body.as_stat_view();

    let response = client
        .put_item()
        .table_name(db_constants::TABLE_NAME)
        .set_item(Some(stat.as_db_item()))
        .send()
        .await;

    match response {
        Ok(success_response) => {
            println!("{:?}", success_response);
            Ok(warp::reply::with_status(
                warp::reply::json(&stat),
                warp::http::StatusCode::OK,
            ))
        }
        Err(e) => {
            println!("{:?}", e);
            Ok(warp::reply::with_status(
                warp::reply::json(&dto::error::Error::new("Error adding stat to the database")),
                warp::http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}
