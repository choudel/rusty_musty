use serde_json::json;
use warp::filters::BoxedFilter;
use warp::{Filter,Reply, Rejection};
use warp::reply::json;


async fn health()->Result<impl Reply,Rejection>{
    Ok(json(&json!({"ok":true})))
    
}
pub(super) fn make_routes()->BoxedFilter<(impl Reply,)>{
    let health =warp::path::end().and_then(health);

    health.boxed()
}