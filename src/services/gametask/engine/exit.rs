use warp::Filter;
use warp::http::StatusCode;
use tokio::sync::Notify;
use std::sync::Arc;

pub fn get_shutdown_signal() -> Arc<Notify> {
    Arc::new(Notify::new())
}

pub fn get_shutdown_route(shutdown_signal: Arc<Notify>) -> impl Filter<Extract = (StatusCode,), Error = warp::Rejection> + Clone {
    warp::path("shutdown")
        .and(warp::post())
        .map(move || {
            shutdown_signal.notify_one();
            StatusCode::OK
        })
}
