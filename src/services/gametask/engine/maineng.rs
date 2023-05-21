use warp::Filter;
use tokio;

use crate::services::gametask::route::basic::fetch_route_obj;
use crate::services::gametask::engine::exit::{get_shutdown_signal, get_shutdown_route};

#[tokio::main]
pub async fn start_gtai_server() {
    println!("Starting server setup...");

    let routes = fetch_route_obj();
    println!("Routes fetched.");

    let shutdown_signal = get_shutdown_signal();
    println!("Shutdown signal created.");

    let shutdown_route = get_shutdown_route(shutdown_signal.clone());
    println!("Shutdown route created.");

    let server = warp::serve(routes.or(shutdown_route));
    println!("Server object created.");

    let (addr, server) = server.bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async move {
        shutdown_signal.notified().await;
    });
    println!("Server bound to address and port with graceful shutdown.");

    println!("Server started at http://{}", addr);

    println!("Server started awaiting connections.");
    server.await;
    println!("Server shut down successfully!!!");
}