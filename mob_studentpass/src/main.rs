mod error;
mod routes;
mod user;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    let routes = routes::routes();
    warp::serve(routes).run(([10, 231, 16, 32], 3030)).await;
}
