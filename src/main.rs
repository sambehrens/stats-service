use std::sync::Arc;

use warp::Filter;

mod db_constants;
mod dto;
mod handlers;
mod time_utils;

type DbClient = Arc<aws_sdk_dynamodb::Client>;

#[tokio::main]
async fn main() {
    let region_provider =
        aws_config::meta::region::RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::from_env()
        .profile_name("stats-db")
        .region(region_provider)
        .load()
        .await;
    let client: DbClient = Arc::new(aws_sdk_dynamodb::Client::new(&config));

    let add_stat = warp::path("stats")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || client.clone()))
        .and_then(handlers::add_stat::add_stat);

    let query_stats = warp::path("stats")
        .and(warp::get())
        .and(warp::query())
        .and_then(handlers::query_stats::query_stats);

    warp::serve(add_stat.or(query_stats))
        .run(([0, 0, 0, 0, 0, 0, 0, 0], 3030))
        .await;
}
