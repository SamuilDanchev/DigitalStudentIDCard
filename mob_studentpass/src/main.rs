mod error;
mod routes;
mod user;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    let routes = routes::routes();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
