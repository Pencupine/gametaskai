use warp::Filter;
use warp::http::{StatusCode, Response};
use warp::{Rejection, Reply};

use crate::services::gametask::cruds_ops::cruds::{read_from_file, write_to_file};
pub fn fetch_route_obj() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let read_route = warp::path("get")
        .and(warp::get())
        .and_then(read_from_file)
        .map(|reply| warp::reply::with_status(reply, StatusCode::OK));

    let write_route = warp::path("post")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(write_to_file)
        .map(|reply| warp::reply::with_status(reply, StatusCode::OK));

    read_route.or(write_route)
}

