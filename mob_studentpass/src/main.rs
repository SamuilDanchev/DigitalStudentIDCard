use lazy_static::lazy_static;
use std::sync::Arc;
use ldap3::{LdapConn, LdapConnAsync, Scope, SearchEntry};
use ldap3::result::Result;

mod db;
mod error;
mod routes;
mod user;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    let pool = db::Db::establish_db_connection().await;
    let pool = Arc::new(pool);
    let routes = routes::routes(pool.clone());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
